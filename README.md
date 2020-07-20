This project is an experimental framework inspired by Microsoft WCF which will allow simplified creation and asynchronous usage of microservices down to the statement level. This means is that each function (and potentially each line) of a program could be composed of calls out to microservices.

The impementation of this framework requires the use of a microservice "broker" that is responsible for URI resolution and potentially acting as a proxy between services, which is itself a microservice.

Initial implementation of the framework should have at least JSON support, however due to the nature of the project, support for Unix domain sockets is planned.


