use crate::operations::Notebook;
use crate::protocols::core::*;
use once_cell::sync::Lazy;
use std::collections::{BTreeMap, HashMap};

pub static DEFAULT_TITLE: Lazy<String> = Lazy::new(|| "Test notebook".to_owned());
pub static TEST_NOTEBOOK: Lazy<Notebook> = Lazy::new(|| {
    let cells = vec![
        Cell::Heading(HeadingCell {
            id: "c1".to_owned(),
            heading_type: HeadingType::H1,
            content: DEFAULT_TITLE.clone(),
            read_only: None,
        }),
        Cell::Heading(HeadingCell {
            id: "c2".to_owned(),
            heading_type: HeadingType::H2,
            content: "Locked subtitle".to_owned(),
            read_only: Some(true),
        }),
        Cell::Text(TextCell {
            id: "c3".to_owned(),
            content: "Some introductory text".to_owned(),
            read_only: None,
        }),
        Cell::Prometheus(PrometheusCell {
            id: "c4".to_owned(),
            content: "go_memstats_alloc_bytes".to_owned(),
            read_only: None,
        }),
        Cell::Graph(GraphCell {
            id: "c5".to_owned(),
            graph_type: GraphType::Line,
            stacking_type: StackingType::None,
            title: "Still unconnected graph".to_owned(),
            source_ids: vec![],
            data: None,
            read_only: None,
            time_range: None,
        }),
        Cell::Prometheus(PrometheusCell {
            id: "c6".to_owned(),
            content: "go_memstats_alloc_bytes".to_owned(),
            read_only: Some(true),
        }),
        Cell::Table({
            let mut data = BTreeMap::new();
            data.insert(
                "c6".to_owned(),
                vec![Instant::new_f64(
                    Metric {
                        name: "go_memstats_alloc_bytes".to_owned(),
                        labels: HashMap::new(),
                    },
                    Point {
                        timestamp: 100.0,
                        value: 1337.0,
                    },
                )],
            );

            TableCell {
                id: "c7".to_owned(),
                source_ids: vec!["c6".to_owned()],
                data: Some(data),
                read_only: None,
            }
        }),
        Cell::ListItem(ListItemCell {
            id: "c8".to_owned(),
            content: "No test *notebook* would be complete without some **Markdown**.\n\
                \n\
                Right before our crown jewel: ***a locked, multi-sourced bar graph with a custom \
                time range***!"
                .to_owned(),
            level: None,
            list_type: ListType::Unordered,
            read_only: Some(true),
        }),
        Cell::Graph({
            let mut data = BTreeMap::new();
            data.insert(
                "c4".to_owned(),
                vec![Series::new_f64(
                    Metric {
                        name: "sourced from c4".to_owned(),
                        labels: HashMap::new(),
                    },
                    vec![
                        Point {
                            timestamp: 50.0,
                            value: 1.0,
                        },
                        Point {
                            timestamp: 100.0,
                            value: 2.0,
                        },
                    ],
                    true,
                )],
            );
            data.insert(
                "c6".to_owned(),
                vec![Series::new_f64(
                    Metric {
                        name: "sourced from c6".to_owned(),
                        labels: HashMap::new(),
                    },
                    vec![
                        Point {
                            timestamp: 50.0,
                            value: 1337.0,
                        },
                        Point {
                            timestamp: 100.0,
                            value: 1337.0,
                        },
                    ],
                    true,
                )],
            );

            GraphCell {
                id: "c9".to_owned(),
                graph_type: GraphType::Bar,
                stacking_type: StackingType::None,
                title: "They call me the crown jewel".to_owned(),
                source_ids: vec!["c4".to_owned(), "c6".to_owned()],
                data: Some(data),
                read_only: Some(true),
                time_range: Some(TimeRange {
                    from: 50.0,
                    to: 150.0,
                }),
            }
        }),
    ];

    let mut data_sources = BTreeMap::new();
    data_sources.insert(
        String::from("inline_data_source_a"),
        NotebookDataSource::Inline(InlineDataSource {
            data_source: DataSource::Prometheus(PrometheusDataSource {
                url: String::from("https://localhost:9000"),
            }),
        }),
    );

    Notebook {
        id: "TEST_NOTEBOOK".to_owned(),
        cells,
        data_sources,
        public: false,
        read_only: false,
        revision: 1,
        time_range: TimeRange {
            from: 0.0,
            to: 100.0,
        },
        title: DEFAULT_TITLE.clone(),
    }
});
