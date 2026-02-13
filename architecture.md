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
