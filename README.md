# C2SD (Controller to Server Distributed Pattern)

This pattern is for any system that needs to stay up and never be completely down.
It is a good match for Micro Blogs, Web Sites, Video Sites or even banks.

### How it works
The system consist of four components, OAuth2 server, Controller, Server and distributed database like CockroachDB. 
These four components can be replicated across multiple platforms: 
1. Private data centers
2. GCP
3. AWS
4. Digital Ocean
5. Others

There can be as many of each component as you need and in as many places as you need.
The system can run on Docker but Kubernetes is not a requirement. The component can be written in any language and run on any server.

### Components
1. OAuth2 Server: A security server and open source GoAuth2 is a good choice.
2. Controller: A system controller that dispatches traffic to different servers.
3. Server: Can be REST servers, data processing servers, video servers or others.
4. Database: A distributed database that should run in different data centers.

### Diagram

![Diagram](/c2sd_pattern.png)

