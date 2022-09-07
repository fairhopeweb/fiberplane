/**
 * @overview The Fiberplane Template library
 * @version 0.1
 */

// Helper functions

local validate = {
  types: [],
  assertType(name, value)::
    if std.member(self.types, std.type(value)) then
      value
    else error 'expected ' + name + ' to be of type: ' + std.join(', ', self.types),
  nullOr:: self + { types+: ['null'] },
  string(name, value):: (self + { types+: ['string'] }).assertType(name, value),
  boolean(name, value):: (self + { types+: ['boolean'] }).assertType(name, value),
  number(name, value):: (self + { types+: ['number'] }).assertType(name, value),
  object(name, value):: (self + { types+: ['object'] }).assertType(name, value),
  array(name, value):: (self + { types+: ['array'] }).assertType(name, value),
};
local matches(a, b, caseSensitive) =
  if !caseSensitive && std.isString(a) && std.isString(b) then
    std.asciiLower(a) == std.asciiLower(b)
  else
    a == b;
local isCell(value) = std.isObject(value) && std.objectHasAll(value, '_class') && value._class == 'CELL';
local isFormattedContent(value) = std.isObject(value) && std.objectHasAll(value, '_class') && value._class == 'FORMATTED_CONTENT';

// Returns the current time as seconds since the Unix epoch
// (including fractions of seconds).
local currentTime() =
  // This is injected by the Fiberplane template runtime.
  // If you are running a template using a stock jsonnet tool
  // and want to use the currentTime function, you must
  // pass in the time (as seconds since the unix epoch)
  // as an ext var
  local time = std.extVar('UNIX_TIMESTAMP');
  if std.isNumber(time) then
    time
  else
    std.parseJson(time);
local relativeTimeRange(minutes) =
  local now = currentTime();
  {
    from: now - 60 * validate.number('minutes', minutes),
    to: now,
  };

/**
 * @class format.FormattedContent
 * @classdesc A class representing formatted text. Each of the formatting functions can be called as methods to append text with the given formatting.
 * @example fp.format.bold('hello ').italic('world')
 */
local formattedContent(content='') =
  // Add content, either as a string, a formatted content object, or an array of strings and/or formatted content objects
  local addContent(fc, content='') =
    if std.type(content) == 'null' then
      fc
    else if std.isString(content) then
      fc {
        content+: content,
      }
    else if isFormattedContent(content) then
      local additionalOffset = std.length(fc.content);
      fc {
        content+: validate.string('content.content', content.content),
        formatting+: std.map(function(f) f { offset+: additionalOffset }, content.formatting),
      }
    else if std.isArray(content) then
      std.foldl(function(formatted, item) addContent(formatted, item), content, fc)
    else
      error 'Invalid content. Expected string, format object, or array. Got: ' + std.toString(content);

  // Helper function to add formatting annotations of a given type and the
  local addContentAndFormatting(fc, content='', format=null, url=null) =
    if std.isString(format) then
      // Add the formatting annotation and the content to fc
      local withStart = fc {
        formatting+: [{
          type: 'start_' + format,
          offset: std.length(fc.content),
          // Only include the url if it is not null
          [if std.isString(url) then 'url']: url,
        }],
      };
      local withContent = addContent(withStart, content);
      // Once the content is added, add the end formatting annotation with the correct offset
      withContent {
        formatting+: [{
          type: 'end_' + format,
          offset: std.length(withContent.content),
        }],
      }
    else
      addContent(fc, content);

  local fc = {
    content: '',
    formatting: [],
    _class:: 'FORMATTED_CONTENT',
    /**
     * Add raw text.
     * Note that if this is added inside a formatting helper, the outer formatting will be applied to this text.
     *
     * @function format.FormattedContent#raw
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @returns {format.FormattedContent}
     */
    raw(content):: addContent(self, content),
    /**
     * Add bold text
     *
     * @function format.FormattedContent#bold
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @returns {format.FormattedContent}
     */
    bold(content):: addContentAndFormatting(self, content, 'bold'),
    /**
     * Add italicized text
     *
     * @function format.FormattedContent#italics
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @returns {format.FormattedContent}
     */
    italics(content):: addContentAndFormatting(self, content, 'italics'),
    /**
     * Add code-formatted text
     *
     * @function format.FormattedContent#code
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @returns {format.FormattedContent}
     */
    code(content):: addContentAndFormatting(self, content, 'code'),
    /**
     * Add highlighted text
     *
     * @function format.FormattedContent#highlight
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @returns {format.FormattedContent}
     */
    highlight(content):: addContentAndFormatting(self, content, 'highlight'),
    /**
     * Add strikethrough text
     *
     * @function format.FormattedContent#strikethrough
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @returns {format.FormattedContent}
     */
    strikethrough(content):: addContentAndFormatting(self, content, 'strikethrough'),
    /**
     * Add underlined text
     *
     * @function format.FormattedContent#underline
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @returns {format.FormattedContent}
     */
    underline(content):: addContentAndFormatting(self, content, 'underline'),
    /**
     * Add a link
     *
     * @function format.FormattedContent#link
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @param {string} url - The URL of the link
     * @returns {format.FormattedContent}
     */
    link(content, url):: addContentAndFormatting(self, content, 'link', url=validate.string('url', url)),
    /**
     * Add a mention
     *
     * @function format.FormattedContent#mention
     * @param {string} userName - The username to mention
     * @param {string} userId - The ID of the user to mention
     * @returns {format.FormattedContent}
     */
    mention(userName, userId):: self {
      content+: '@' + userId,
      formatting+: [{
        type: 'mention',
        name: userName,
        userId: userId
      }],
    },
    /**
     * Add a timestamp
     *
     * @function format.FormattedContent#timestamp
     * @param {string} timestamp - The RFC3339-formatted timestamp to add
     * @returns {format.FormattedContent}
     */
    timestamp(timestamp):: self {
      content+: timestamp,
      formatting+: [{
        type: 'timestamp',
        timestamp: timestamp,
      }],
    },
  };
  addContent(fc, content);

/**
 * @class notebook.Notebook
 * @classdesc A Fiberplane Notebook.
 *
 * @see {@link notebook.new notebook\.new} to create a Notebook
 */
local notebook = {
  /**
   * Create a new notebook with the given title.
   *
   * @function notebook.new
   * @memberof notebook
   * @param title
   * @returns {notebook.Notebook}
   */
  new(title):: {
    title: validate.string('title', title),
    timeRange: relativeTimeRange(minutes=60),
    dataSources: {},
    labels: [],
    cells: [],
    // This is used to generate the cell IDs in the addCell
    // method. It does not appear in the JSON output
    _nextCellId:: 1,

    /**
     * Set the notebook time range relative to when it is created.
     *
     * For example, specifying `minutes=60` will set the start timestamp
     * to 60 minutes before the notebook is created. The end timestamp
     * will automatically be set to the time when the notebook is created.
     *
     * By default, the time range is set to 60 minutes relative to when the notebook is created.
     *
     * @function notebook.Notebook#setTimeRangeRelative
     * @param {number} minutes
     * @returns {notebook.Notebook}
     */
    setTimeRangeRelative(minutes):: self {
      timeRange+: relativeTimeRange(minutes),
    },

    /**
     * Set the time range of the notebook using absolute timestamps.
     *
     * Note: in most cases, you will want to use {@link notebook#setTimeRangeRelative} instead.
     *
     * @function notebook.Notebook#setTimeRangeAbsolute
     * @param {number} from - Starting timestamp in seconds since the Unix epoch
     * @param {number} to - Ending timestamp in seconds since the Unix epoch
     * @returns {notebook.Notebook}
     */
    setTimeRangeAbsolute(from, to):: self {
      timeRange+: {
        from: validate.number('from', from),
        to: validate.number('to', to),
      },
    },

    /**
     * Add a data source from a connected Fiberplane Proxy.
     *
     * When the notebook is created from this template, the template runtime makes
     * available the list of proxies and their data sources. This function adds
     * one of those data sources and allows you to filter data sources by various
     * search terms. You may specify any combination of search terms.
     *
     * @function notebook.Notebook#addProxyDataSource
     * @param {string | null} type=null - Add a data source of this type (e.g. `'prometheus'` or `'elasticsearch'`)
     * @param {string | null} name=null - Add a data source with this name (e.g. `'Production Prometheus'`)
     * @param {string | null} proxyName=null - Add a data source from this proxy (e.g. `'production'`)
     * @param {string | null} proxyId=null - Add a data source from this proxy (e.g. `'a1bc701f-1f0e-4d4a-9ad0-e4ee54f17102'`)
     * @param {string | null} alias=null - Optionally override the name of this data source
     *  (mostly useful if you have multiple data sources in the same notebook with the same name)
     * @param {boolean} caseSensitive=false - Whether to use case sensitive matching for the above search terms
     * @param {boolean} errorIfMultipleMatch=false - Error if there are multiple data sources matching the search terms.
     *  By default it will add one of the matching data sources.
     * @param {boolean} errorIfNone=false - Error if there are no data sources matching the search terms.
     *  By default it will simply not add a data source.
     * @returns {notebook.Notebook}
     *
     * @example <caption>Adding a proxy data source by type</caption>
     * notebook.addProxyDataSource(type='prometheus')
     * @example <caption>Adding a proxy data source by proxy and data source name</caption>
     * notebook.addProxyDataSource(proxyName='production', name='Production Prometheus')
     */
    addProxyDataSource(
      alias=null,
      type=null,
      name=null,
      proxyId=null,
      proxyName=null,
      caseSensitive=false,
      errorIfMultipleMatch=false,
      errorIfNoneMatch=false
    )::
      // Find the data source(s) that match the given criteria
      local dataSources = std.filter(
        function(dataSource)
          (!std.isString(type) || matches(type, dataSource.type, caseSensitive))
          && (!std.isString(name) || matches(name, dataSource.name, caseSensitive))
          && (!std.isString(proxyId) || matches(proxyId, dataSource.proxy.id, caseSensitive))
          && (!std.isString(proxyName) || matches(proxyName, dataSource.proxy.name, caseSensitive)),
        if std.isString(std.extVar('PROXY_DATA_SOURCES')) then
          std.parseJson(std.extVar('PROXY_DATA_SOURCES'))
        else std.extVar('PROXY_DATA_SOURCES')
      );

      // Add the data source or error if a configured condition is violated
      if errorIfMultipleMatch && std.length(dataSources) > 1 then
        error 'Multiple data sources match criteria'
      else if errorIfNoneMatch && std.length(dataSources) == 0 then
        error 'No data sources match criteria'
      else if std.length(dataSources) == 0 then
        self
      else
        local _alias = if std.isString(alias) then alias else dataSources[0].name;
        self {
          dataSources+: {
            [_alias]: {
              type: 'inline',
              dataSource: {
                type: 'proxy',
                proxyId: dataSources[0].proxy.id,
                dataSourceName: dataSources[0].name,
                dataSourceType: dataSources[0].type,
              },
            },
          },
        },

    /**
     * Add a direct data source (one that is accessible on the internet) to the notebook.
     *
     * @function notebook.Notebook#addDirectDataSource
     * @param {string} name Data source name
     * @param {string} type Data source type
     * @param {object} config Data source config.
     * @returns {notebook.Notebook}
     *
     * @example <caption>Adding a data source with type prometheus</caption>
     * notebook.addDirectDataSource(
     *   name='Production Prometheus',
     *   type='prometheus',
     *   config={
     *     url='https://user:password@prometheus.example.com'
     *   },
     * )
     * @example <caption>Adding a data source with type elasticsearch</caption>
     * notebook.addDirectDataSource(
     *   name='Production Elasticsearch',
     *   type='elasticsearch',
     *   config={
     *     url='https://elasticsearch.example.com',
     *     timestampFieldNames: ['@timestamp'],
     *     bodyFieldNames: ['message'],
     *   },
     * )
     * @example <caption>Adding a data source with type loki</caption>
     * notebook.addDirectDataSource(
     *   name='Production Loki',
     *   type='loki',
     *   config={
     *     url='https://loki.example.com'
     *   },
     *)
     */
    addDirectDataSource(name, type, config={}, options={}, url=null):: self {
      local dataSource = if std.isString(config) then
        // Old signature was `addDirectDataSource(name, type, url, options={})`
        options {
          type: type,
          url: config,
        }
      else if std.isString(url) then
        // Support the old parameter 'url' as a named argument
        options {
          type: type,
          url:
            std.trace('The `url` parameter is deprecated. Use `config.url` instead.', url),
        }
      else
        config {
          type: type,
        },

      dataSources+: {
        [name]: {
          type: 'inline',
          dataSource: dataSource,
        },
      },
    },

    /**
     * Add a single cell to the notebook.
     *
     * @function notebook.Notebook#addCell
     * @param {cell.Cell} cell
     * @returns {notebook.Notebook}
     */
    addCell(cell)::
      local cellId = self._nextCellId;
      // Remove all null values and add the id field as a string
      local cellWithId = std.prune(cell) + {
        id: cellId + '',
      };
      self {
        _nextCellId: cellId + 1,
        // Append the cell to the cells array
        cells+: [cellWithId],
      },

    /**
     * Add an array of cells to the notebook.
     *
     * Note: this function supports nested arrays of cells.
     *
     * @function notebook.Notebook#addCells
     * @param {cell.Cell[]} cells
     * @returns {notebook.Notebook}
     */
    addCells(cells)::
      // Call addCell for each cell in the array
      // and recursively call addCells if there
      // are nested arrays
      std.foldl(function(n, cell) (
        if std.isArray(cell) then
          n.addCells(cell)
        else n.addCell(cell)
      ), validate.array('cells', cells), self),

    /**
     * Add a single label to the notebook.
     *
     * @function notebook.Notebook#addLabel
     * @param {string} key - Key of the label
     * @param {string} value - Value of the label
     * @returns {notebook.Notebook}
     *
     * @example notebook.addLabel(key='service', value='api')
     */
    addLabel(key, value=''):: self {
      labels+: [{
        key: validate.string('key', key),
        value: validate.string('value', value),
      }],
    },

    /**
     * Add an object of labels to the notebook.
     *
     * @function notebook.Notebook#addLabels
     * @param {object} labels - Map of keys and values
     * @returns {notebook.Notebook}
     *
     * @example notebook.addLabels({
     *  service: 'api',
     *  severity: 'high'
     * })
     */
    addLabels(labels):: std.foldl(
      function(nb, key)
        local value = if std.isString(labels[key]) then labels[key] else '';
        nb.addLabel(key, value),
      std.objectFields(validate.object('labels', labels)),
      self
    ),
  },
};

/**
 * @class cell.Cell
 * @classdesc An individual cell in a notebook
 */
local cell = {
  // Base type that cells are built from.
  // Each cell-specific function will merge other
  // fields into the object returned here.
  local base = function(type, content, readOnly)
    formattedContent(content) + {
      type: type,
      readOnly: validate.nullOr.boolean('readOnly', readOnly),
      _class:: 'CELL',
      /**
       * Lock the cell
       *
       * @method cell.Cell#setReadOnly
       * @param {boolean} readOnly=true
       * @returns {cell.Cell}
       */
      setReadOnly(readOnly=true):: self {
        readOnly: readOnly,
      },
    },

  // List item
  local li = function(listType, content, startNumber, level, readOnly)
    base('list_item', content, readOnly) + {
      listType: listType,
      level: validate.nullOr.number('level', level),
      startNumber: validate.nullOr.number('startNumber', startNumber),
    },

  // Function to create a list from an array of strings, cells, and/or other lists.
  // It sets the startNumber field for all list items.
  // If it also sets the level field for all nested list items.
  local list = function(listType, cells, startNumber, level, readOnly)
    std.foldl(
      function(accumulator, content)
        // Treat strings as list items and increment the start number
        if std.isString(content) || isFormattedContent(content) then
          local cell = li(listType, content, accumulator.startNumber, level, readOnly);
          // Merge these values into the accumulator
          // (the + operator is optional when merging objects)
          accumulator {
            startNumber+: 1,
            array+: [cell],
          }
        else if std.isArray(content) then
          // Nested lists need to have their level incremented
          local nextLevel = if std.isNumber(level) then level + 1 else 1;
          accumulator {
            array+: list(listType, content, 1, nextLevel, readOnly),
          }
        else if isCell(content) then
          // Add the cell to the array and update the level if the cell is a list item
          local cellWithLevel = content {
            [if content.type == 'list_item' then 'level']: level,
          };
          accumulator {
            array+: [cellWithLevel],
          }
        else error 'Expected a string, formatted content, cell, or array of those. Got: ' + std.toString(content),
      cells,
      { startNumber: startNumber, array: [] },
    ).array,

  /**
   * Create a checkbox cell
   *
   * @function cell.checkbox
   * @param {boolean} checked=false - Whether the checkbox is checked
   * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
   * @param {number | null} level=null - Specify the indentation level.
   *  The top level is `null`, the first indented level is `1`, and so on
   * @param {boolean} readOnly=false - Whether the cell is locked
   * @returns {cell.Cell}
   */
  checkbox(content='', checked=false, level=null, readOnly=null)::
    base('checkbox', content, readOnly) + {
      checked: validate.boolean('checked', checked),
      level: validate.nullOr.number('level', level),
    },

  /**
   * Create a code cell
   *
   * @function cell.code
   * @param {boolean} checked=false - Whether the checkbox is checked
   * @param {string} content='' - Cell text content
   * @param {string | null} syntax=null - Specify the syntax to use for rendering the code
   * @param {boolean} readOnly=false - Whether the cell is locked
   * @returns {cell.Cell}
   */
  code(content='', syntax=null, readOnly=null)::
    base('code', validate.string('content', content), readOnly) + {
      syntax: validate.nullOr.string('syntax', syntax),
    },
  /**
   * Create a divider (horizontal rule) cell
   *
   * @function cell.divider
   * @param {boolean} readOnly=false - Whether the cell is locked
   * @returns {cell.Cell}
   */
  divider(readOnly=null)::
    base('divider', null, readOnly),

  /**
   * Heading cells
   * @namespace cell.heading
   */
  heading:: {
    local h = function(headingType, content, readOnly)
      base('heading', content, readOnly) + {
        headingType: headingType,
      },

    /**
     * Create an H1 cell
     *
     * Also accessible as `cell.h1`
     *
     * @function cell.heading.h1
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @param {boolean} readOnly=false - Whether the cell is locked
     * @returns {cell.Cell}
     */
    h1(content='', readOnly=null):: h('h1', content, readOnly),

    /**
     * Create an H2 cell
     *
     * Also accessible as `cell.h2`
     *
     * @function cell.heading.h2
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @param {boolean} readOnly=false - Whether the cell is locked
     * @returns {cell.Cell}
     */
    h2(content='', readOnly=null):: h('h2', content, readOnly),

    /**
     * Create an H3 cell
     *
     * Also accessible as `cell.h3`
     *
     * @function cell.heading.h3
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @param {boolean} readOnly=false - Whether the cell is locked
     * @returns {cell.Cell}
     */
    h3(content='', readOnly=null):: h('h3', content, readOnly),
  },
  h1:: cell.heading.h1,
  h2:: cell.heading.h2,
  h3:: cell.heading.h3,

  /**
    * Helper functions for easily creating lists
    *
    * @namespace cell.list
    */
  list:: {
    /**
     * Create an ordered list
     *
     * Also accessible as `cell.ol` and `cell.orderedList`
     *
     * @function cell.list.ordered
     * @param {Array.<(string | cell.Cell | Array)>} cells An array of strings, cells, or nested lists.
     *  Strings will become numbered list items. Other cell types are included as they are.
     *  Nested lists have their indentation `level` automatically incremented.
     * @param {number} startNumber=1 Starting number for the whole list. This function automatically handles
     *  numbering for all items in this list.
     * @param {number | null} level=null - Specify the indentation level.
     *  The top level is `null`, the first indented level is `1`, and so on
     * @param {boolean} readOnly=false - Whether the cells are locked
     * @returns {cell.Cell[]}
     */
    ordered(cells=[], startNumber=1, level=null, readOnly=null)::
      list('ordered', cells, startNumber, level, readOnly),

    /**
     * Create an unordered list
     *
     * Also accessible as `cell.ul` and `cell.unorderedList`
     *
     * @function cell.list.unordered
     * @param {Array.<(string | cell.Cell | Array)>} cells An array of strings, cells, or nested lists.
     *  Strings will become list items. Other cell types are included as they are.
     *  Nested lists have their indentation `level` automatically incremented.
     * @param {number | null} level=null - Specify the indentation level.
     *  The top level is `null`, the first indented level is `1`, and so on
     * @param {boolean} readOnly=false - Whether the cells are locked
     * @returns {cell.Cell[]}
     */
    unordered(cells=[], startNumber=1, level=null, readOnly=null)::
      list('unordered', cells, startNumber, level, readOnly),
  },
  ul:: cell.list.unordered,
  unorderedList:: cell.list.unordered,
  ol:: cell.list.ordered,
  orderedList:: cell.list.ordered,

  /**
    * Individual list items.
    *
    * In most cases, you will want to use {@link Cell.list} instead.
    *
    * @namespace cell.listItem
    */
  listItem:: {

    /**
     * Create an ordered list item
     *
     * @function cell.listItem.ordered
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @param {number | null} startNumber=null - Specify the starting number.
     *  Mostly useful if you want to start the list at a number other than `1`.
     * @param {number | null} level=null - Specify the indentation level.
     *  The top level is `null`, the first indented level is `1`, and so on
     * @param {boolean} readOnly=false - Whether the cell is locked
     * @returns {cell.Cell}
     */
    ordered(content='', level=null, startNumber=null, readOnly=null)::
      li('ordered', content, startNumber, level, readOnly),

    /**
     * Create an unordered list item
     *
     * @function cell.listItem.unordered
     * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
     * @param {number | null} level=null - Specify the indentation level.
     *  The top level is `null`, the first indented level is `1`, and so on
     * @param {boolean} readOnly=false - Whether the cell is locked
     * @returns {cell.Cell}
     */
    unordered(content='', level=null, startNumber=null, readOnly=null)::
      li('unordered', content, startNumber, level, readOnly),
  },

  /**
   * Create a Prometheus query cell
   *
   * @function cell.prometheus
   * @param {string} content='' - Cell text content
   * @param {boolean} readOnly=false - Whether the cell is locked
   * @returns {cell.Cell}
   */
  prometheus(content='', readOnly=null):: base('prometheus', validate.string('content', content), readOnly),

  /**
   * Create an Elasticsearch query cell
   *
   * @function cell.elasticsearch
   * @param {string} content='' - Cell text content
   * @param {boolean} readOnly=false - Whether the cell is locked
   * @returns {cell.Cell}
   */
  elasticsearch(content='', readOnly=null):: base('elasticsearch', validate.string('content', content), readOnly),

  /**
   * Create a Loki query cell
   *
   * @function cell.loki
   * @param {string} content='' - Cell text content
   * @param {boolean} readOnly=false - Whether the cell is locked
   * @returns {cell.Cell}
   */
  loki(content='', readOnly=null):: base('loki', validate.string('content', content), readOnly),

  /**
   * Create a plain text cell
   *
   * @function cell.text
   * @param {string | format.FormattedContent | Array.<(string | format.FormattedContent)>} content - The content to add
   * @param {boolean} readOnly=false - Whether the cell is locked
   * @returns {cell.Cell}
   */
  text(content='', readOnly=null):: base('text', content, readOnly),

  /**
   * Create an image cell
   *
   * @function cell.image
   * @param {string} url - URL of the image
   * @param {boolean} readOnly=false - Whether the cell is locked
   */
  image(url=null, readOnly=null)::
    base('image', '', readOnly) + {
      content: null,
      url: validate.nullOr.string('url', url),
    },
};

// Library exports
{
  /**
   * Functions for creating Fiberplane Notebooks
   * @namespace notebook
   *
   * @example fp.notebook.new('My Notebook')
   *  .setTimeRangeRelative(minutes=60)
   *  .addCells([...])
   */
  notebook: notebook,
  /**
   * Functions for creating notebook cells
   * @namespace cell
   *
   * @example <caption>Adding cells to a notebook</caption>
   * notebook.addCells([
   *   cell.h1('Title'),
   *   cell.text('Hello world!'),
   *   // See below for all of the available cell types
   * ])
   */
  cell: cell,
  /**
   * Functions for formatting text
   *
   * @namespace format
   *
   * @example fp.format.bold('hello')
   * @example <caption>Nested formatting</caption>
   * fp.format.bold(fp.format.italic('hello'))
   * @example <caption>Creating a cell with different text formats</caption>
   * fp.cell.text(['hello ', fp.format.bold('world '), fp.format.italics('!')])
   * // This is equivalent to:
   * fp.cell.text(fp.format.raw('hello ').bold('world ').italics('!'))
   *
   * @borrows format.FormattedContent#raw as raw
   * @borrows format.FormattedContent#bold as bold
   * @borrows format.FormattedContent#code as code
   * @borrows format.FormattedContent#highlight as highlight
   * @borrows format.FormattedContent#italics as italics
   * @borrows format.FormattedContent#underline as underline
   * @borrows format.FormattedContent#strikethrough as strikethrough
   * @borrows format.FormattedContent#link as link
   */
  format: formattedContent(),
}
