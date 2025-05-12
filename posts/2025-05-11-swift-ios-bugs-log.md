---
layout: post
title: Swift iOS Bugs Log
date: May 11, 2025
categories:
  - code
description: log of bugs encountered while using Swift on iOS 
keywords: 2025, software
author: Matt
tags:
  - software-engineering
  - swift
  - ios
  - bugs
---


## Swiftdata: Duplicate version checksums detected

iOS 18.4 simulator and device crashing with the following error:

```
*** Terminating app due to uncaught exception 'NSInvalidArgumentException', reason: 'Duplicate version checksums detected.'
*** First throw call stack:
``` 


This occurred when running swiftdata schema migrations.


### Solution

Namespace all data models in versioned schema.
Avoid sharing models across different versions of the schema, even if they are identical.

example:

```swift
// Version 1
enum ModelSchemaV1: VersionedSchema {
  static var versionIdentifier: Schema.Version {
    Schema.Version(1, 0, 0)
  }
}

extension ModelSchemaV1 {
  @Model
  class DataModel: Decodable {
    @Attribute(.unique) var id: String
  }
}

// Version 2
enum ModelSchemaV2: VersionedSchema {
  static var versionIdentifier: Schema.Version {
    Schema.Version(2, 0, 0)
  }
}

extension ModelSchemaV2 {
  @Model
  class DataModel: Decodable {
    @Attribute(.unique) var id: String
  }
}
```