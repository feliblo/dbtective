---
title: Node Checks
type: docs
prev: docs/Checks
sidebar:
  open: true
---

dbtective's node checks apply rules to so-called dbt nodes. You might not be familiar with the term, you have probably worked with nodes!

In dbt, a **node** is any resource defined in your project that participates in your data pipeline.  [documentation](https://docs.getdbt.com/reference/node-selection/syntax)
Nodes include:

- **models** transform raw data into usable tables or views
- **tests** validate data quality and logic
- **sources** connect to external/raw data
- **seeds** load static reference data
- **snapshots** track slowly changing dimensions over time
- **exposures** document downstream uses like dashboards or apps
- **analyses** store ad-hoc queries and reports

## Overview of Node checks
