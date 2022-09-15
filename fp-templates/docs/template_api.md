## Objects

<dl>
<dt><a href="#notebook">notebook</a> : <code>object</code></dt>
<dd><p>Functions for creating Fiberplane Notebooks</p></dd>
<dt><a href="#cell">cell</a> : <code>object</code></dt>
<dd><p>Functions for creating notebook cells</p></dd>
<dt><a href="#format">format</a> : <code>object</code></dt>
<dd><p>Functions for formatting text</p></dd>
</dl>

<a name="notebook"></a>

## notebook : <code>object</code>
<p>Functions for creating Fiberplane Notebooks</p>

**Kind**: global namespace  
**Example**  
```js
fp.notebook.new('My Notebook')
 .setTimeRangeRelative(minutes=60)
 .addCells([...])
```

* [notebook](#notebook) : <code>object</code>
    * [.Notebook](#notebook.Notebook)
        * [.setTimeRangeRelative(minutes)](#notebook.Notebook+setTimeRangeRelative) ⇒ [<code>Notebook</code>](#notebook.Notebook)
        * [.setTimeRangeAbsolute(from, to)](#notebook.Notebook+setTimeRangeAbsolute) ⇒ [<code>Notebook</code>](#notebook.Notebook)
        * [.addProxyDataSource(type, name, proxyName, proxyId, alias, caseSensitive, errorIfMultipleMatch, errorIfNone)](#notebook.Notebook+addProxyDataSource) ⇒ [<code>Notebook</code>](#notebook.Notebook)
        * [.addDirectDataSource(name, type, config)](#notebook.Notebook+addDirectDataSource) ⇒ [<code>Notebook</code>](#notebook.Notebook)
        * [.addCell(cell)](#notebook.Notebook+addCell) ⇒ [<code>Notebook</code>](#notebook.Notebook)
        * [.addCells(cells)](#notebook.Notebook+addCells) ⇒ [<code>Notebook</code>](#notebook.Notebook)
        * [.addLabel(key, value)](#notebook.Notebook+addLabel) ⇒ [<code>Notebook</code>](#notebook.Notebook)
        * [.addLabels(labels)](#notebook.Notebook+addLabels) ⇒ [<code>Notebook</code>](#notebook.Notebook)
    * [.new(title)](#notebook.new) ⇒ [<code>Notebook</code>](#notebook.Notebook)

<a name="notebook.Notebook"></a>

### notebook.Notebook
<p>A Fiberplane Notebook.</p>

**Kind**: static class of [<code>notebook</code>](#notebook)  
**See**: [notebook\.new](#notebook.new) to create a Notebook  

* [.Notebook](#notebook.Notebook)
    * [.setTimeRangeRelative(minutes)](#notebook.Notebook+setTimeRangeRelative) ⇒ [<code>Notebook</code>](#notebook.Notebook)
    * [.setTimeRangeAbsolute(from, to)](#notebook.Notebook+setTimeRangeAbsolute) ⇒ [<code>Notebook</code>](#notebook.Notebook)
    * [.addProxyDataSource(type, name, proxyName, proxyId, alias, caseSensitive, errorIfMultipleMatch, errorIfNone)](#notebook.Notebook+addProxyDataSource) ⇒ [<code>Notebook</code>](#notebook.Notebook)
    * [.addDirectDataSource(name, type, config)](#notebook.Notebook+addDirectDataSource) ⇒ [<code>Notebook</code>](#notebook.Notebook)
    * [.addCell(cell)](#notebook.Notebook+addCell) ⇒ [<code>Notebook</code>](#notebook.Notebook)
    * [.addCells(cells)](#notebook.Notebook+addCells) ⇒ [<code>Notebook</code>](#notebook.Notebook)
    * [.addLabel(key, value)](#notebook.Notebook+addLabel) ⇒ [<code>Notebook</code>](#notebook.Notebook)
    * [.addLabels(labels)](#notebook.Notebook+addLabels) ⇒ [<code>Notebook</code>](#notebook.Notebook)

<a name="notebook.Notebook+setTimeRangeRelative"></a>

#### notebook.setTimeRangeRelative(minutes) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Set the notebook time range relative to when it is created.</p>
<p>For example, specifying <code>minutes=60</code> will set the start timestamp
to 60 minutes before the notebook is created. The end timestamp
will automatically be set to the time when the notebook is created.</p>
<p>By default, the time range is set to 60 minutes relative to when the notebook is created.</p>

**Kind**: instance method of [<code>Notebook</code>](#notebook.Notebook)  

| Param | Type |
| --- | --- |
| minutes | <code>number</code> | 

<a name="notebook.Notebook+setTimeRangeAbsolute"></a>

#### notebook.setTimeRangeAbsolute(from, to) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Set the time range of the notebook using absolute timestamps.</p>
<p>Note: in most cases, you will want to use [notebook#setTimeRangeRelative](notebook#setTimeRangeRelative) instead.</p>

**Kind**: instance method of [<code>Notebook</code>](#notebook.Notebook)  

| Param | Type | Description |
| --- | --- | --- |
| from | <code>number</code> | <p>Starting timestamp in seconds since the Unix epoch</p> |
| to | <code>number</code> | <p>Ending timestamp in seconds since the Unix epoch</p> |

<a name="notebook.Notebook+addProxyDataSource"></a>

#### notebook.addProxyDataSource(type, name, proxyName, proxyId, alias, caseSensitive, errorIfMultipleMatch, errorIfNone) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Add a data source from a connected Fiberplane Proxy.</p>
<p>When the notebook is created from this template, the template runtime makes
available the list of proxies and their data sources. This function adds
one of those data sources and allows you to filter data sources by various
search terms. You may specify any combination of search terms.</p>

**Kind**: instance method of [<code>Notebook</code>](#notebook.Notebook)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| type | <code>string</code> \| <code>null</code> | <code>null</code> | <p>Add a data source of this type (e.g. <code>'prometheus'</code> or <code>'elasticsearch'</code>)</p> |
| name | <code>string</code> \| <code>null</code> | <code>null</code> | <p>Add a data source with this name (e.g. <code>'Production Prometheus'</code>)</p> |
| proxyName | <code>string</code> \| <code>null</code> | <code>null</code> | <p>Add a data source from this proxy (e.g. <code>'production'</code>)</p> |
| proxyId | <code>string</code> \| <code>null</code> | <code>null</code> | <p>Add a data source from this proxy (e.g. <code>'a1bc701f-1f0e-4d4a-9ad0-e4ee54f17102'</code>)</p> |
| alias | <code>string</code> \| <code>null</code> | <code>null</code> | <p>Optionally override the name of this data source (mostly useful if you have multiple data sources in the same notebook with the same name)</p> |
| caseSensitive | <code>boolean</code> | <code>false</code> | <p>Whether to use case sensitive matching for the above search terms</p> |
| errorIfMultipleMatch | <code>boolean</code> | <code>false</code> | <p>Error if there are multiple data sources matching the search terms. By default it will add one of the matching data sources.</p> |
| errorIfNone | <code>boolean</code> | <code>false</code> | <p>Error if there are no data sources matching the search terms. By default it will simply not add a data source.</p> |

**Example** *(Adding a proxy data source by type)*  
```js
notebook.addProxyDataSource(type='prometheus')
```
**Example** *(Adding a proxy data source by proxy and data source name)*  
```js
notebook.addProxyDataSource(proxyName='production', name='Production Prometheus')
```
<a name="notebook.Notebook+addDirectDataSource"></a>

#### notebook.addDirectDataSource(name, type, config) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Add a direct data source (one that is accessible on the internet) to the notebook.</p>

**Kind**: instance method of [<code>Notebook</code>](#notebook.Notebook)  

| Param | Type | Description |
| --- | --- | --- |
| name | <code>string</code> | <p>Data source name</p> |
| type | <code>string</code> | <p>Data source type</p> |
| config | <code>object</code> | <p>Data source config.</p> |

**Example** *(Adding a data source with type prometheus)*  
```js
notebook.addDirectDataSource(
  name='Production Prometheus',
  type='prometheus',
  config={
    url='https://user:password@prometheus.example.com'
  },
)
```
**Example** *(Adding a data source with type elasticsearch)*  
```js
notebook.addDirectDataSource(
  name='Production Elasticsearch',
  type='elasticsearch',
  config={
    url='https://elasticsearch.example.com',
    timestampFieldNames: ['@timestamp'],
    bodyFieldNames: ['message'],
  },
)
```
**Example** *(Adding a data source with type loki)*  
```js
notebook.addDirectDataSource(
  name='Production Loki',
  type='loki',
  config={
    url='https://loki.example.com'
  },
)
```
<a name="notebook.Notebook+addCell"></a>

#### notebook.addCell(cell) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Add a single cell to the notebook.</p>

**Kind**: instance method of [<code>Notebook</code>](#notebook.Notebook)  

| Param | Type |
| --- | --- |
| cell | [<code>Cell</code>](#cell.Cell) | 

<a name="notebook.Notebook+addCells"></a>

#### notebook.addCells(cells) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Add an array of cells to the notebook.</p>
<p>Note: this function supports nested arrays of cells.</p>

**Kind**: instance method of [<code>Notebook</code>](#notebook.Notebook)  

| Param | Type |
| --- | --- |
| cells | [<code>Array.&lt;Cell&gt;</code>](#cell.Cell) | 

<a name="notebook.Notebook+addLabel"></a>

#### notebook.addLabel(key, value) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Add a single label to the notebook.</p>

**Kind**: instance method of [<code>Notebook</code>](#notebook.Notebook)  

| Param | Type | Description |
| --- | --- | --- |
| key | <code>string</code> | <p>Key of the label</p> |
| value | <code>string</code> | <p>Value of the label</p> |

**Example**  
```js
notebook.addLabel(key='service', value='api')
```
<a name="notebook.Notebook+addLabels"></a>

#### notebook.addLabels(labels) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Add an object of labels to the notebook.</p>

**Kind**: instance method of [<code>Notebook</code>](#notebook.Notebook)  

| Param | Type | Description |
| --- | --- | --- |
| labels | <code>object</code> | <p>Map of keys and values</p> |

**Example**  
```js
notebook.addLabels({
 service: 'api',
 severity: 'high'
})
```
<a name="notebook.new"></a>

### notebook.new(title) ⇒ [<code>Notebook</code>](#notebook.Notebook)
<p>Create a new notebook with the given title.</p>

**Kind**: static method of [<code>notebook</code>](#notebook)  

| Param |
| --- |
| title | 

<a name="cell"></a>

## cell : <code>object</code>
<p>Functions for creating notebook cells</p>

**Kind**: global namespace  
**Example** *(Adding cells to a notebook)*  
```js
notebook.addCells([
  cell.h1('Title'),
  cell.text('Hello world!'),
  // See below for all of the available cell types
])
```

* [cell](#cell) : <code>object</code>
    * [.Cell](#cell.Cell)
        * [.setReadOnly(readOnly)](#cell.Cell+setReadOnly) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.heading](#cell.heading) : <code>object</code>
        * [.h1(content, readOnly)](#cell.heading.h1) ⇒ [<code>Cell</code>](#cell.Cell)
        * [.h2(content, readOnly)](#cell.heading.h2) ⇒ [<code>Cell</code>](#cell.Cell)
        * [.h3(content, readOnly)](#cell.heading.h3) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.list](#cell.list) : <code>object</code>
        * [.ordered(cells, startNumber, level, readOnly)](#cell.list.ordered) ⇒ [<code>Array.&lt;Cell&gt;</code>](#cell.Cell)
        * [.unordered(cells, level, readOnly)](#cell.list.unordered) ⇒ [<code>Array.&lt;Cell&gt;</code>](#cell.Cell)
    * [.listItem](#cell.listItem) : <code>object</code>
        * [.ordered(content, startNumber, level, readOnly)](#cell.listItem.ordered) ⇒ [<code>Cell</code>](#cell.Cell)
        * [.unordered(content, level, readOnly)](#cell.listItem.unordered) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.checkbox(checked, content, level, readOnly)](#cell.checkbox) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.code(checked, content, syntax, readOnly)](#cell.code) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.divider(readOnly)](#cell.divider) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.prometheus(content, readOnly)](#cell.prometheus) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.elasticsearch(content, readOnly)](#cell.elasticsearch) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.loki(content, readOnly)](#cell.loki) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.text(content, readOnly)](#cell.text) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.image(url, readOnly)](#cell.image)

<a name="cell.Cell"></a>

### cell.Cell
<p>An individual cell in a notebook</p>

**Kind**: static class of [<code>cell</code>](#cell)  
<a name="cell.Cell+setReadOnly"></a>

#### cell.setReadOnly(readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Lock the cell</p>

**Kind**: instance method of [<code>Cell</code>](#cell.Cell)  

| Param | Type | Default |
| --- | --- | --- |
| readOnly | <code>boolean</code> | <code>true</code> | 

<a name="cell.heading"></a>

### cell.heading : <code>object</code>
<p>Heading cells</p>

**Kind**: static namespace of [<code>cell</code>](#cell)  

* [.heading](#cell.heading) : <code>object</code>
    * [.h1(content, readOnly)](#cell.heading.h1) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.h2(content, readOnly)](#cell.heading.h2) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.h3(content, readOnly)](#cell.heading.h3) ⇒ [<code>Cell</code>](#cell.Cell)

<a name="cell.heading.h1"></a>

#### heading.h1(content, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create an H1 cell</p>
<p>Also accessible as <code>cell.h1</code></p>

**Kind**: static method of [<code>heading</code>](#cell.heading)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> |  | <p>The content to add</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.heading.h2"></a>

#### heading.h2(content, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create an H2 cell</p>
<p>Also accessible as <code>cell.h2</code></p>

**Kind**: static method of [<code>heading</code>](#cell.heading)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> |  | <p>The content to add</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.heading.h3"></a>

#### heading.h3(content, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create an H3 cell</p>
<p>Also accessible as <code>cell.h3</code></p>

**Kind**: static method of [<code>heading</code>](#cell.heading)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> |  | <p>The content to add</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.list"></a>

### cell.list : <code>object</code>
<p>Helper functions for easily creating lists</p>

**Kind**: static namespace of [<code>cell</code>](#cell)  

* [.list](#cell.list) : <code>object</code>
    * [.ordered(cells, startNumber, level, readOnly)](#cell.list.ordered) ⇒ [<code>Array.&lt;Cell&gt;</code>](#cell.Cell)
    * [.unordered(cells, level, readOnly)](#cell.list.unordered) ⇒ [<code>Array.&lt;Cell&gt;</code>](#cell.Cell)

<a name="cell.list.ordered"></a>

#### list.ordered(cells, startNumber, level, readOnly) ⇒ [<code>Array.&lt;Cell&gt;</code>](#cell.Cell)
<p>Create an ordered list</p>
<p>Also accessible as <code>cell.ol</code> and <code>cell.orderedList</code></p>

**Kind**: static method of [<code>list</code>](#cell.list)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| cells | <code>Array.&lt;(string\|cell.Cell\|Array)&gt;</code> |  | <p>An array of strings, cells, or nested lists. Strings will become numbered list items. Other cell types are included as they are. Nested lists have their indentation <code>level</code> automatically incremented.</p> |
| startNumber | <code>number</code> | <code>1</code> | <p>Starting number for the whole list. This function automatically handles numbering for all items in this list.</p> |
| level | <code>number</code> \| <code>null</code> | <code></code> | <p>Specify the indentation level. The top level is <code>null</code>, the first indented level is <code>1</code>, and so on</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cells are locked</p> |

<a name="cell.list.unordered"></a>

#### list.unordered(cells, level, readOnly) ⇒ [<code>Array.&lt;Cell&gt;</code>](#cell.Cell)
<p>Create an unordered list</p>
<p>Also accessible as <code>cell.ul</code> and <code>cell.unorderedList</code></p>

**Kind**: static method of [<code>list</code>](#cell.list)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| cells | <code>Array.&lt;(string\|cell.Cell\|Array)&gt;</code> |  | <p>An array of strings, cells, or nested lists. Strings will become list items. Other cell types are included as they are. Nested lists have their indentation <code>level</code> automatically incremented.</p> |
| level | <code>number</code> \| <code>null</code> | <code></code> | <p>Specify the indentation level. The top level is <code>null</code>, the first indented level is <code>1</code>, and so on</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cells are locked</p> |

<a name="cell.listItem"></a>

### cell.listItem : <code>object</code>
<p>Individual list items.</p>
<p>In most cases, you will want to use [Cell.list](Cell.list) instead.</p>

**Kind**: static namespace of [<code>cell</code>](#cell)  

* [.listItem](#cell.listItem) : <code>object</code>
    * [.ordered(content, startNumber, level, readOnly)](#cell.listItem.ordered) ⇒ [<code>Cell</code>](#cell.Cell)
    * [.unordered(content, level, readOnly)](#cell.listItem.unordered) ⇒ [<code>Cell</code>](#cell.Cell)

<a name="cell.listItem.ordered"></a>

#### listItem.ordered(content, startNumber, level, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create an ordered list item</p>

**Kind**: static method of [<code>listItem</code>](#cell.listItem)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> |  | <p>The content to add</p> |
| startNumber | <code>number</code> \| <code>null</code> | <code></code> | <p>Specify the starting number. Mostly useful if you want to start the list at a number other than <code>1</code>.</p> |
| level | <code>number</code> \| <code>null</code> | <code></code> | <p>Specify the indentation level. The top level is <code>null</code>, the first indented level is <code>1</code>, and so on</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.listItem.unordered"></a>

#### listItem.unordered(content, level, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create an unordered list item</p>

**Kind**: static method of [<code>listItem</code>](#cell.listItem)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> |  | <p>The content to add</p> |
| level | <code>number</code> \| <code>null</code> | <code></code> | <p>Specify the indentation level. The top level is <code>null</code>, the first indented level is <code>1</code>, and so on</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.checkbox"></a>

### cell.checkbox(checked, content, level, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create a checkbox cell</p>

**Kind**: static method of [<code>cell</code>](#cell)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| checked | <code>boolean</code> | <code>false</code> | <p>Whether the checkbox is checked</p> |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> |  | <p>The content to add</p> |
| level | <code>number</code> \| <code>null</code> | <code></code> | <p>Specify the indentation level. The top level is <code>null</code>, the first indented level is <code>1</code>, and so on</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.code"></a>

### cell.code(checked, content, syntax, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create a code cell</p>

**Kind**: static method of [<code>cell</code>](#cell)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| checked | <code>boolean</code> | <code>false</code> | <p>Whether the checkbox is checked</p> |
| content | <code>string</code> | <code>&quot;&#x27;&#x27;&quot;</code> | <p>Cell text content</p> |
| syntax | <code>string</code> \| <code>null</code> | <code>null</code> | <p>Specify the syntax to use for rendering the code</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.divider"></a>

### cell.divider(readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create a divider (horizontal rule) cell</p>

**Kind**: static method of [<code>cell</code>](#cell)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.prometheus"></a>

### cell.prometheus(content, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create a Prometheus query cell</p>

**Kind**: static method of [<code>cell</code>](#cell)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> | <code>&quot;&#x27;&#x27;&quot;</code> | <p>Cell text content</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.elasticsearch"></a>

### cell.elasticsearch(content, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create an Elasticsearch query cell</p>

**Kind**: static method of [<code>cell</code>](#cell)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> | <code>&quot;&#x27;&#x27;&quot;</code> | <p>Cell text content</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.loki"></a>

### cell.loki(content, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create a Loki query cell</p>

**Kind**: static method of [<code>cell</code>](#cell)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> | <code>&quot;&#x27;&#x27;&quot;</code> | <p>Cell text content</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.text"></a>

### cell.text(content, readOnly) ⇒ [<code>Cell</code>](#cell.Cell)
<p>Create a plain text cell</p>

**Kind**: static method of [<code>cell</code>](#cell)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> |  | <p>The content to add</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="cell.image"></a>

### cell.image(url, readOnly)
<p>Create an image cell</p>

**Kind**: static method of [<code>cell</code>](#cell)  

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| url | <code>string</code> |  | <p>URL of the image</p> |
| readOnly | <code>boolean</code> | <code>false</code> | <p>Whether the cell is locked</p> |

<a name="format"></a>

## format : <code>object</code>
<p>Functions for formatting text</p>

**Kind**: global namespace  
**Example**  
```js
fp.format.bold('hello')
```
**Example** *(Nested formatting)*  
```js
fp.format.bold(fp.format.italic('hello'))
```
**Example** *(Creating a cell with different text formats)*  
```js
fp.cell.text(['hello ', fp.format.bold('world '), fp.format.italics('!')])
// This is equivalent to:
fp.cell.text(fp.format.raw('hello ').bold('world ').italics('!'))
```

* [format](#format) : <code>object</code>
    * [.FormattedContent](#format.FormattedContent)
        * [.raw(content)](#format.FormattedContent+raw) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.bold(content)](#format.FormattedContent+bold) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.italics(content)](#format.FormattedContent+italics) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.code(content)](#format.FormattedContent+code) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.highlight(content)](#format.FormattedContent+highlight) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.strikethrough(content)](#format.FormattedContent+strikethrough) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.underline(content)](#format.FormattedContent+underline) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.link(content, url)](#format.FormattedContent+link) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.mention(userName, userId)](#format.FormattedContent+mention) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.timestamp(timestamp)](#format.FormattedContent+timestamp) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
        * [.label(key, url)](#format.FormattedContent+label) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.raw(content)](#format.raw) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.bold(content)](#format.bold) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.code(content)](#format.code) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.highlight(content)](#format.highlight) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.italics(content)](#format.italics) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.underline(content)](#format.underline) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.strikethrough(content)](#format.strikethrough) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.link(content, url)](#format.link) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)

<a name="format.FormattedContent"></a>

### format.FormattedContent
<p>A class representing formatted text. Each of the formatting functions can be called as methods to append text with the given formatting.</p>

**Kind**: static class of [<code>format</code>](#format)  

* [.FormattedContent](#format.FormattedContent)
    * [.raw(content)](#format.FormattedContent+raw) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.bold(content)](#format.FormattedContent+bold) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.italics(content)](#format.FormattedContent+italics) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.code(content)](#format.FormattedContent+code) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.highlight(content)](#format.FormattedContent+highlight) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.strikethrough(content)](#format.FormattedContent+strikethrough) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.underline(content)](#format.FormattedContent+underline) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.link(content, url)](#format.FormattedContent+link) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.mention(userName, userId)](#format.FormattedContent+mention) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.timestamp(timestamp)](#format.FormattedContent+timestamp) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
    * [.label(key, url)](#format.FormattedContent+label) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)

<a name="format.FormattedContent+raw"></a>

#### formattedContent.raw(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add raw text.
Note that if this is added inside a formatting helper, the outer formatting will be applied to this text.</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.FormattedContent+bold"></a>

#### formattedContent.bold(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add bold text</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.FormattedContent+italics"></a>

#### formattedContent.italics(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add italicized text</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.FormattedContent+code"></a>

#### formattedContent.code(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add code-formatted text</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.FormattedContent+highlight"></a>

#### formattedContent.highlight(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add highlighted text</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.FormattedContent+strikethrough"></a>

#### formattedContent.strikethrough(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add strikethrough text</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.FormattedContent+underline"></a>

#### formattedContent.underline(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add underlined text</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.FormattedContent+link"></a>

#### formattedContent.link(content, url) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add a link</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |
| url | <code>string</code> | <p>The URL of the link</p> |

<a name="format.FormattedContent+mention"></a>

#### formattedContent.mention(userName, userId) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add a mention</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| userName | <code>string</code> | <p>The username to mention</p> |
| userId | <code>string</code> | <p>The ID of the user to mention</p> |

<a name="format.FormattedContent+timestamp"></a>

#### formattedContent.timestamp(timestamp) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add a timestamp</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| timestamp | <code>string</code> | <p>The RFC3339-formatted timestamp to add</p> |

<a name="format.FormattedContent+label"></a>

#### formattedContent.label(key, url) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add a label</p>

**Kind**: instance method of [<code>FormattedContent</code>](#format.FormattedContent)  

| Param | Type | Description |
| --- | --- | --- |
| key | <code>string</code> | <p>The label's key</p> |
| url | <code>string</code> | <p>The label's value (optional)</p> |

<a name="format.raw"></a>

### format.raw(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add raw text.
Note that if this is added inside a formatting helper, the outer formatting will be applied to this text.</p>

**Kind**: static method of [<code>format</code>](#format)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.bold"></a>

### format.bold(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add bold text</p>

**Kind**: static method of [<code>format</code>](#format)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.code"></a>

### format.code(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add code-formatted text</p>

**Kind**: static method of [<code>format</code>](#format)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.highlight"></a>

### format.highlight(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add highlighted text</p>

**Kind**: static method of [<code>format</code>](#format)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.italics"></a>

### format.italics(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add italicized text</p>

**Kind**: static method of [<code>format</code>](#format)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.underline"></a>

### format.underline(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add underlined text</p>

**Kind**: static method of [<code>format</code>](#format)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.strikethrough"></a>

### format.strikethrough(content) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add strikethrough text</p>

**Kind**: static method of [<code>format</code>](#format)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |

<a name="format.link"></a>

### format.link(content, url) ⇒ [<code>FormattedContent</code>](#format.FormattedContent)
<p>Add a link</p>

**Kind**: static method of [<code>format</code>](#format)  

| Param | Type | Description |
| --- | --- | --- |
| content | <code>string</code> \| [<code>FormattedContent</code>](#format.FormattedContent) \| <code>Array.&lt;(string\|format.FormattedContent)&gt;</code> | <p>The content to add</p> |
| url | <code>string</code> | <p>The URL of the link</p> |

