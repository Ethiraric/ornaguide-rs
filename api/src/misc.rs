/// Generate a `post_impl` function that takes the given filter and getter as parameter and
/// applies the filters to the entities returned by the getter.
/// This is the main body for each route:
///   - Get the array we're interested in from the `OrnaData`
///   - Apply filters (if there are)
///   - Apply sorting (if there is)
///   - Convert to JSON
/// The function has the following signature:
/// `fn (mut $filter_type) -> Result<serde_json::Value, $crate::error::Error>`
#[macro_export]
macro_rules! make_post_impl {
    ($filter_type:ty) => {
        /// Implementation function just so I can return a `Result` and `?`.
        pub fn post_impl(
            mut filters: $filter_type,
        ) -> Result<serde_json::Value, $crate::error::Error> {
            let options = filters.options.extract();
            with_data(|data| {
                if filters.is_none() {
                    Ok(<$filter_type>::get_entities(data).clone())
                } else {
                    let filters = filters.compiled()?.into_fn_vec();
                    Ok(<$filter_type>::get_entities(data)
                        .iter()
                        .filter(|entity| filters.iter().map(|f| f(entity)).all(|x| x))
                        .cloned()
                        .collect_vec())
                }
            })
            .and_then(|mut entity| {
                <$filter_type>::apply_sort(&options, &mut entity)?;
                Ok(entity)
            })
            .and_then(|entities| {
                serde_json::to_value(entities)
                    .map_err(ornaguide_rs::error::Error::from)
                    .to_internal_server_error()
            })
            .and_then(|mut entities| {
                if options.deref {
                    with_data(|data| <$filter_type>::deref(&mut entities, data))?
                }
                Ok(entities)
            })
        }
    };
}
