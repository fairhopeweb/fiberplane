# Fiberplane Templates

> Programmatically generate Fiberplane Notebooks for repeatable workflows.

## Overview

Fiberplane Templates are built with the [Jsonnet](https://jsonnet.org/) data templating language.

This repository includes:

- Fiberplane [Jsonnet library](./fiberplane.libsonnet) with functions for creating Notebooks ([API Docs](./docs/template_api.md))
- [Example Templates](./examples) for various use cases (incident response, root cause analysis, etc)
- Rust library for expanding Templates into Notebooks and for converting existing Notebooks into Templates

## Quickstart

The [Fiberplane CLI](https://github.com/fiberplane/fp) is the recommended way to interact with Templates (see the [docs](https://github.com/fiberplane/fp#templates) or run `fp help templates`).

## Structure of a Template

Most Fiberplane Templates export a Jsonnet function that accepts some parameters and creates a Notebook using the helper functions provided by the Fiberplane Jsonnet library.

```jsonnet
local fp = import 'fiberplane.libsonnet';
local c = fp.cell;
local fmt = fp.format;

// Parameters are named and can have default values
function(incidentName='API Outage')
  fp.notebook
    .new('Incident Response for: ' + incidentName)
    .setTimeRangeRelative(minutes=60)
    .addCells([
      // The library exposes helper functions for creating every cell type
      c.h1('Heading'),
      c.text(
        // There are also helper functions for formatting text
        fmt.bold('Hello World!')
      )
    ])
```

See the [examples](./examples) for more detailed, use case-specific Templates.

## [Template API Documentation](./docs/template_api.md)

See the generated API docs [here](./docs/template_api.md).

## Development

### VS Code

If you want to edit Jsonnet files in VS Code, you can use the [Jsonnet NG](https://marketplace.visualstudio.com/items?itemName=Sebbia.jsonnetng) extension.

You should add the following to your VS Code `settings.json` file to edit Template files without it showing errors.
This includes the Fiberplane Jsonnet library and external variables normally provided by the Template expansion functions.

```json
{
  "jsonnet.libPaths": ["path/to/fiberplane/templates/"],
  "jsonnet.extStrs": {
    "PROXY_DATA_SOURCES": "[]"
  }
}
```

### Running Tests

To run the tests (including the examples), run:

```shell
cargo test --lib --examples
```

### Generating Documentation

The Jsonnet library API documentation is generated from [JSDoc](https://jsdoc.app/) comments in [fiberplane.libsonnet](./fiberplane.libsonnet) using [jsdoc-to-markdown](https://github.com/jsdoc2md/jsdoc-to-markdown).

To (re)generate the documentation, you can use this Docker command:

```shell
docker run --rm -v $PWD:$PWD node:17 npx -y jsdoc-to-markdown -c $PWD/jsdoc.json $PWD/fiberplane.libsonnet > docs/template_api.md
```

Alternatively, you can use Node.js directly by using the following command:

```shell
npx -y jsdoc-to-markdown -c jsdoc.json fiberplane.libsonnet > docs/template_api.md
```
