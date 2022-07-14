## Introduction

### Document Overview

This document will be written in chronological order, appending new sections for each step in the design process. Previous sections will for the most part not be modified except where otherwise noted.

### Domain-Driven Design, CQRS, and Event Sourcing

This process and resultant architecture will feature elements of domain-driven design (DDD), but the project is primarily an exercise in CQRS architecture with event sourcing as a persistence mechanism. As such, familiar DDD terms may be seen which are used loosely or incorrectly.

### The App

This project represents the backend for a hypothetical food ordering service similar to DoorDash or Uber Eats. Users are able to act as either drivers or customers from within the same mobile app. The app services only a single neighborhood with a small selection of restaurants.

## Step 1: Define Ubiquitious Language

The first step will be to define a domain language that can be used by both the developer(s) and the (hypothetical) business. I'll attempt to avoid ambiguity and inconsistency, e.g. using terms like "checkpoint" rather than "event" which might lead to confusion. The first draft of the ubiquitious language (UB) for this project will be written before development begins, but will be updated as my understanding of the domain grows.

_This glossary will be kept updated with terms from the current domain language:_

### Cart

A list of one or more items from a particular partner that a customer intends to submit as an order.

### Checkpoint

An event connected to a given order which can be used to notify both customer and driver about the status of the order.

### Customer

The user which created the order.

### Driver

The user which will be picking up and delivering the order.

### Item

An entry on an order, made up of an offering and a quantity requested.

### Order

An attempt made by a customer to purchase one or more item(s) from a partner.

### Offering

A product offered for sale by a partner which can be added as an item on an order.

### Partner

A merchant at a single location which offers items that can be ordered by customers. This can also refer to a user who acts on behalf of the partnered business.

### User

A person who has created an account and can act as either customer or driver.

## Step 2: Define Requirements

### Customer Requirements

Request

- see a list of their orders.
- see a detailed view of any order.
- see information about a partner and a list of their offerings.

Command

- create a new empty cart.
- add items to their cart.
- submit an order with the items in their cart.
- rate their experience with both the partner and driver.

### Driver Requirements

Request

- see a list of currently available orders.

Command

- confirm that they have accepted an order.
- confirm that an order has been picked up.
- confirm that an order has been delivered.

### Partner Requirements

Request

- see a list of orders the partner has received.
- see a detailed view of an order.

Command

- confirm that an order has been sent to the kitchen.

## Step 3: Define Domain Models

The domain model will map the terms of the UB onto objects and their relationships. This will be an anemic domain model with little or no business logic contained on the objects themselves, which is considered by some to be an anti-pattern. I will try to avoid common pitfalls caused by anemic domain models by making the models immutable and instead applying business logic such as validations and calculations through a functional approach.

This model will be defined in terms of domain relationships without regard to persistence concerns. In fact, persistence in this project will be acheived entirely through event sourcing.
