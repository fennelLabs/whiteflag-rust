Problem:  I have the sr25519 scheme implemented in our software, however sr25519 is not a defined scheme for the JSON Web Signatures for WF Authentication Method 1, so we have to deviate there from the applicable RFC and probably document that we only use the JWT/JWS data structure. However, that means a manual implementation or otherwise existing libraries will complain or just not do the signatures. What do you think?

Whiteflag: https://api.whiteflagprotocol.org/md/openapi.html

Replies:
What needs to be done?

I’m not sure yet. Depends on if I can find a reasonable implementation of something Whiteflag supports in Rust which was the problem that led us where we ended up in the first place. 

We’re trying to find whether there’s a working and supported signature system that fits the signatures allowed on Whiteflag messages by the specification. 

The library isn’t so important as the signature scheme. We need to find something that matches Whiteflag which isn’t provided by the crypto library we’re using for that section of the code. 