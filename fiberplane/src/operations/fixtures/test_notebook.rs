use crate::{
    operations::{Notebook, NotebookVisibility},
    protocols::core::*,
    protocols::formatting::{Annotation, AnnotationWithOffset, Formatting},
};
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use time::{macros::datetime, OffsetDateTime};

pub static DEFAULT_TITLE: Lazy<String> = Lazy::new(|| "Test notebook".to_owned());
pub static TEST_NOTEBOOK: Lazy<Notebook> = Lazy::new(|| {
    let cells = vec![
        Cell::Heading(HeadingCell {
            id: "c1".to_owned(),
            heading_type: HeadingType::H1,
            content: DEFAULT_TITLE.clone(),
            formatting: Some(Formatting::default()),
            read_only: None,
        }),
        Cell::Heading(HeadingCell {
            id: "c2".to_owned(),
            heading_type: HeadingType::H2,
            content: "Locked subtitle".to_owned(),
            formatting: Some(Formatting::default()),
            read_only: Some(true),
        }),
        Cell::Text(TextCell {
            id: "c3".to_owned(),
            content: "Some introductory text".to_owned(),
            formatting: Some(Formatting::default()),
            read_only: None,
        }),
        Cell::Loki(LokiCell {
            id: "c4".to_owned(),
            content: "go_memstats_alloc_bytes".to_owned(),
            read_only: None,
        }),
        Cell::Graph(GraphCell {
            id: "c5".to_owned(),
            data_links: Vec::new(),
            graph_type: GraphType::Line,
            read_only: None,
            stacking_type: StackingType::None,
        }),
        Cell::Provider(ProviderCell {
            id: "c6".to_owned(),
            formatting: Some(Formatting::default()),
            intent: "prometheus,x-instants".to_owned(),
            output: Some(vec![Cell::Table(TableCell {
                id: "c6/table".to_owned(),
                read_only: None,
                rows: vec![TableRow {
                    cols: vec![
                        TableColumn {
                            formatting: None,
                            text: "go_memstats_alloc_bytes".to_owned(),
                        },
                        TableColumn {
                            formatting: None,
                            text: "1337.0".to_owned(),
                        },
                    ],
                }],
            })]),
            query_data: Some(
                "application/x-www-form-urlencoded,query=go_memstats_alloc_bytes".to_owned(),
            ),
            read_only: Some(true),
            response: None,
            title: "Table".to_owned(),
        }),
        Cell::Discussion(DiscussionCell {
            id: "c7".to_owned(),
            thread_id: "123".to_owned(),
            read_only: None,
        }),
        Cell::ListItem(ListItemCell {
            id: "c8".to_owned(),
            content: "No test *notebook* would be complete without some **Markdown**.\n\
                \n\
                Right before our crown jewel: ***a locked, multi-sourced bar graph with a custom \
                time range***!"
                .to_owned(),
            formatting: Some(vec![
                AnnotationWithOffset::new(8, Annotation::StartItalics),
                AnnotationWithOffset::new(18, Annotation::EndItalics),
                AnnotationWithOffset::new(50, Annotation::StartBold),
                AnnotationWithOffset::new(62, Annotation::EndBold),
                AnnotationWithOffset::new(95, Annotation::StartBold),
                AnnotationWithOffset::new(95, Annotation::StartItalics),
                AnnotationWithOffset::new(159, Annotation::EndBold),
                AnnotationWithOffset::new(159, Annotation::EndItalics),
            ]),
            level: None,
            list_type: ListType::Unordered,
            read_only: Some(true),
            start_number: None,
        }),
        Cell::Provider(ProviderCell {
            id: "c9".to_owned(),
            formatting: None,
            intent: "prometheus,metrics".to_owned(),
            output: Some(vec![Cell::Graph(GraphCell {
                id: "c9/graph".to_owned(),
                data_links: vec![
                    "cell-data:application/vnd.fiberplane.metrics,c6".to_owned(),
                    "cell-data:application/vnd.fiberplane.metrics,c9".to_owned(),
                ],
                graph_type: GraphType::Bar,
                read_only: None,
                stacking_type: StackingType::None,
            })]),
            query_data: Some(
                "application/x-www-form-urlencoded,\
                    query=go_memstats_alloc_bytes&\
                    time_range=2022-08-31T11:00:00.000Z+2022-08-31T12:00:00.000Z"
                    .to_owned(),
            ),
            read_only: Some(true),
            response: None,
            title: "They call me the crown jewel".to_owned(),
        }),
        Cell::Elasticsearch(ElasticsearchCell {
            id: "c10".to_owned(),
            content: "kubernetes.labels.app:api".to_owned(),
            read_only: None,
        }),
        Cell::Log(LogCell {
            id: "c11".to_owned(),
            title: "Logs".to_owned(),
            source_ids: vec!["c10".to_owned()],
            time_range: Some(LegacyTimeRange {
                from: 50.0,
                to: 150.0,
            }),
            ..Default::default()
        }),
        Cell::Text(TextCell {
            id: "c12".to_owned(),
            content: "italic bold both".to_owned(),
            formatting: Some(vec![
                AnnotationWithOffset {
                    annotation: Annotation::StartItalics,
                    offset: 0,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndItalics,
                    offset: 6,
                },
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 7,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 11,
                },
                AnnotationWithOffset {
                    annotation: Annotation::StartItalics,
                    offset: 12,
                },
                AnnotationWithOffset {
                    annotation: Annotation::StartBold,
                    offset: 12,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndItalics,
                    offset: 16,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndBold,
                    offset: 16,
                },
            ]),
            read_only: None,
        }),
        Cell::Text(TextCell {
            id: "c13".to_owned(),
            content: "🇳🇱 and https://fiberplane.com".to_owned(),
            formatting: Some(vec![
                AnnotationWithOffset {
                    annotation: Annotation::StartLink {
                        url: "https://fiberplane.com".to_owned(),
                    },
                    offset: 7,
                },
                AnnotationWithOffset {
                    annotation: Annotation::EndLink,
                    offset: 29,
                },
            ]),
            read_only: None,
        }),
        Cell::Provider(ProviderCell {
            id: "c14".to_owned(),
            formatting: Some(Vec::new()),
            intent: "sentry;my-data-source,x-error-details".to_owned(),
            output: None,
            query_data: Some("application/x-www-form-urlencoded,trace_id=123".to_owned()),
            read_only: None,
            response: None,
            title: "".to_owned(),
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
        read_only: false,
        revision: 1,
        time_range: TimeRange {
            from: datetime!(2022-09-19 8:00 UTC).into(),
            to: datetime!(2022-09-19 9:00 UTC).into(),
        },
        title: DEFAULT_TITLE.clone(),
        visibility: NotebookVisibility::Private,
        created_at: OffsetDateTime::UNIX_EPOCH,
        updated_at: OffsetDateTime::UNIX_EPOCH,
        created_by: CreatedBy {
            user_type: UserType::Individual,
            name: "name".to_string(),
        },
        labels: vec![Label::new("existing-key", "existing-value")],
    }
});
