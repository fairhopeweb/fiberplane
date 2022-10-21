local fp = import '../../fiberplane.libsonnet';
local c = fp.cell;
local fmt = fp.format;

function(incidentName)
  fp.notebook
  .new("Incident: '" + incidentName + "'")
  .setDataSourceForProviderType('prometheus', 'prometheus', 'dev')
  .setTimeRangeRelative(minutes=60)
  .addCells([
    c.text(fmt.raw("Let's ").italics('debug').raw(' this ').bold('incident!').raw(' ').label('foo', 'bar').raw(' ').label('baz')),
    c.heading.h2('TODOs:', readOnly=true),
    c.checkbox('Investigate'),
    c.code('// Some code to run
let a = \'b\';
let b = "c";'),
    c.checkbox('Resolve'),
    c.checkbox('Profit'),
    c.heading.h2('Hypotheses', readOnly=true),
    c.loki('loki query'),
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
    c.prometheus(content='http_requests', title='sample title'),
    c.prometheus(),
  ])
  .addLabel('key1')
  .addLabel('key2', 'value2')
  .addLabels({
    key3: '',
    key4: 'value4',
    key5: null,
  })
