# Omni's architecture

## Constraints

Omni is a middleware that sends requests to APIs on behalf of the clients, and attaches authentication to the requests.
To achieve that, Omni stores the secrets (credentials, access tokens, etc), and they must remain confidential.
So the main security constraint is confidentiality.

We want to keep these secure from local AI agents.
They are on the same machine, as the same user, they can run arbitrary code, and download ready-made malwares.
They can come in two flavors: running as an user, or running as an admin.

Omni's architecture's goal is to have the API secrets completely unaccessible from user-space malwares,
and very hard to access from admin malware.

Additionally, clients authenticate with Omni, and have permissions to perform certain actions.
The security constraints for this are mostly integrity and a bit of confidentiality.

For these two, availability is less of a priority:

- if an AI agent is to stop working, it can be re-launched
- if the credentials are wiped from the database, the user can generate new ones

## Performing requests

Omni performs requests on behalf of the client. To achieve that, the client sends a request to Omni (secured by OAuth),
then Omni sends the corresponding request to the third party API via HTTPS, injecting the authentication.
When the API responds, Omni forwards the response to the client.

Omni supports all of HTTP as an input, and forwards it as-is (except for the authentication part).
Then, there are two parts that can be customized: how to build the connection data, and how to attach this
connection data to the outgoing request. Also, this connection data should be stored securely.
While there are standards for authorization (OAuth2, Basic auth, etc), some APIs don't conform to them.

So we will run Lua code for building connection data, and for attaching connection data to requests:

- A Lua function will be called each time a request is sent by Omni, to add headers (or anything else) to the request.
- Another Lua function will be called each time an user want to create a connection to an API.
- Additionally, there may be other Lua functions for rotating the tokens. These are needed for refresh token flows.

Each connector has a set of Lua functions, and the connection data is tied to the connector.
Lua functions handle connection data, that are sensitive. Consequently, Lua functions should be curated, and Omni must
make sure they aren't modified (an attacker could modify the Lua functions to leak the connection data).
To achieve that, users "install" the Lua scripts in Omni. Omni then stores a hash of the scripts and checks the Lua
scripts' hash before executing them.

## Connecting to Omni

Clients connect to Omni to send request. Each client has permissions to perform requests using some connections.
We also want to audit which clients performed what requests.
So we need to authenticate clients. We use OAuth, with the client credentials flow.
This plays nicely with the two-database system: we don't have to store the client secret, we can just store a KDF.

Even though we bother using authentication for clients, we should keep in mind that agents will typically store the
client secret on disk, where it can be read by other clients.
So clients can easily impersonate each other, and the permissions are not secure.
This is basically security by obfuscation, aka no security.

In a typical OAuth2 client credentials fashion, the client first asks for an access token, then uses this access token
in requests.
As access token generation is cheap, they will have a very short lifespan, of 1 minute.

Additionally, to manage connections, authenticate connections to the API, manage clients, and their permissions,
Omni provides a management API.
It is authenticated using access tokens. Clients ask for an access token via the API, which then prompts the user
for confirmation (Windows Hello, etc). Once the user confirms, an access token is sent to the client.
This authorization process is OAuth2 compatible, but not OAuth2 compliant.

## The database

The main challenge with Omni is its database: we must store a database on a machine that is possibly compromised.
Classic crypto primitives don't work: malwares have the same rights as Omni, or even higher rights, in the same machine,
logged-in as the same user.

We then have to be quite creative to prevent malicious AI agents from reading our database (breaking confidentiality)
or to tamper with it (breaking integrity).

Modern hardware and OS offer ways to prevent users from reading the secrets. TPM, VBS, Secure Enclave, etc. These are,
however, not magic and come with trade-offs that must be accounted-for.

For all platforms, the database is encrypted using a decryption key (DEK).
The DEK is encrypted using a TPM key (or equivalent).
The encrypted DEK is stored on disk, alongside the encrypted database.
This way, the TPM is only used once to decrypt the DEK, not every time we need to decrypt the database.
