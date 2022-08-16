
## CQRS and Event Sourcing Example 

This is a hypothetical example of how an ecommerce web backend might be implemented using CQRS end event sourcing. This project focuses on the flow of creating and modifying a shopping cart. 

Requests are partitioned into commands (and events) or queries in the application layer. Commands and events are both processed through a message bus which operates as a queue. Queries access resources by reading a collection of correlated events and replaying them to retrieve the state of resource at a given point in time. A separate endpoint can be used to retrieve raw events, which might be useful for (for example) a customer service tool.

For simplicity, both commands and queries operate on the same mongodb connection, but this could easily be extended to split the persistence layer into separate read and write databases.

Product queries (as opposed to user and cart queries) do not operate on events, as an example of how event sourcing can be used alongside more conventional approaches for storing data.




