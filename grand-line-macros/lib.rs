#[macro_export]
macro_rules! optional_enum {
    ($v:expr => $t:path) => {
        if let $t(e) = $v {
            Some(e)
        } else {
            None
        }
    };
}
#[macro_export]
macro_rules! optional_enum_ref {
    ($v:expr => $t:path) => {
        if let $t(ref mut e) = $v {
            Some(e)
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! unwrap_enum {
    ($v:expr => $t:path) => {
        if let $t(e) = $v {
            e
        } else {
            panic!("not a {}", stringify!($t))
        }
    };
}
#[macro_export]
macro_rules! unwrap_enum_ref {
    ($v:expr => $t:path) => {
        if let $t(ref mut e) = $v {
            e
        } else {
            panic!("not a {}", stringify!($t))
        }
    };
}

#[macro_export]
macro_rules! field_quote {
    ($($v:tt)*) => {
        syn::parse::Parser::parse2(syn::Field::parse_named, quote!($($v)*)).unwrap()
    };
}

#[macro_export]
macro_rules! order_by {
    ($model:ident[$($order_by:tt),*]) => {
        grand_line::internal::paste! {
            Some(vec![$([<$model OrderBy>]::$order_by),*])
        }
    };
}

#[macro_export]
macro_rules! quick_serve_axum {
    ($schema:ident) => {
        grand_line::internal::quick_serve_axum!(
            $schema,
            Schema = MySchema,
            port = 4000,
            path = "/api/graphql"
        )
    };
    ($schema:ident, Schema=$Schema:ident, port=$port:literal, path=$path:literal) => {
        async fn graphql(
            axum::Extension(schema): axum::Extension<$Schema>,
            req: async_graphql_axum::GraphQLRequest,
        ) -> async_graphql_axum::GraphQLResponse {
            schema.execute(req.into_inner()).await.into()
        }
        async fn graphiql() -> impl axum::response::IntoResponse {
            axum::response::Html(
                async_graphql::http::GraphiQLSource::build()
                    .endpoint($path)
                    .finish(),
            )
        }
        let app = axum::Router::new()
            .route($path, axum::routing::get(graphiql).post(graphql))
            .layer(axum::Extension($schema))
            .into_make_service();
        println!("serving on http://localhost:{}{}", $port, $path);
        axum::serve(
            tokio::net::TcpListener::bind(format!("127.0.0.1:{}", $port))
                .await
                .unwrap(),
            app,
        )
        .await
        .unwrap();
    };
}
