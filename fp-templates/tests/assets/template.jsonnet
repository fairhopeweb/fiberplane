local fp = import '../../fiberplane.libsonnet';
local c = fp.cell;
local fmt = fp.format;

function(incidentName)
  fp.notebook
  .new("Incident: '" + incidentName + "'")
  .setTimeRangeAbsolute(from=1639239669.739, to=1639239729)
  .addDirectDataSource(
    name='direct elasticsearch',
    type='elasticsearch',
    config={
      url: 'https://elasticsearch.dev.fiberplane.io',
      timestampFieldNames: ['@timestamp'],
      bodyFieldNames: ['message'],
    },
  )
  .addDirectDataSource(
    name='direct prometheus',
    type='prometheus',
    config={
      url: 'https://prometheus.dev.fiberplane.io'
    },
  )
  .addCells([
    c.text(fmt.raw("Let's ").italics('debug').raw(' this ').bold('incident!')),
    c.heading.h2('TODOs:', readOnly=true),
    c.checkbox('Investigate'),
    c.code('// Some code to run
let a = \'b\';
let b = "c";'),
    c.checkbox('Resolve'),
    c.checkbox('Profit'),
    c.heading.h2('Hypotheses', readOnly=true),
    c.prometheus('prometheus query'),
    c.list.ordered([
      'Step 1',
      c.code('Some code'),
      'Step 2',
      c.list.unordered([
        'Bullet 1',
        'Bullet 2',
      ]),
    ]),
    c.image(url='http://example.com/image.png'),
  ])
  .addLabel('key1')
  .addLabel('key2', 'value2')
  .addLabels({
    key3: '',
    key4: 'value4',
    key5: null,
  })
