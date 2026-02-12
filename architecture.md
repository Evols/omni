# Omni's architecture

## Constraints

Omni is a client that sends requests to APIs, and attaches authentication to the requests. To achieve that,
Omni stores the secrets (credentials, access tokens, etc), and they must remain confidential. So the main security constraint
is confidentiality.

We want to keep these secure from local AI agents.
They are on the same machine, as the same user, they can run arbitrary code, and download ready-made malwares.
They can come in two flavors: running as an user, or running as an admin.

Omni's architecture's goal is to have the API secrets completely unaccessible from user-space malwares,
and very hard to access from admin malware.

Additionally, the communication and authentication of clients to Omni must be secured at the state of the art.
