# How to Build a Module

## What's This All About?

ZARK-WAF Core is built to be super flexible. You can add new features by making your own modules. This guide will show you how to whip up a new module for ZARK-WAF Core.

## What You Need

- Rust programming language (grab the latest stable version)
- Cargo package manager
- A basic idea of how ZARK-WAF Core works

## Steps to Make Your Own Module

### 1. Create the ZarkModule Trait

First things first, make a new Rust file for your module. You'll need to implement the `ZarkModule` trait. This trait is like a blueprint that all ZARK-WAF modules have to follow.

### 2. Use the ZarkMessenger Module

ZARK-WAF Core comes with a `zark_messenger` module. It's how your custom module can chat with other modules in the system.

Cool stuff about the `zark_messenger` module:
- It uses a publish-subscribe pattern (like a group chat)
- It's asynchronous (doesn't hold things up)
- It's type-safe (so you don't accidentally send the wrong kind of message)

To use `zark_messenger` in your module:
1. Import the stuff you need from `zark_messenger`
2. Use its methods to send messages or listen for specific types of messages
3. Handle any incoming messages in your module's logic
