use sudograph::graphql_database;

graphql_database!("canisters/graphql/src/schema.graphql");


#[sudograph::ic_cdk_macros::update]
async fn graphql_mutation_custom(query: String, variables: String) -> String {
    //comment out for local dev (playground has no IC auth)
    //let principals= vec![
        //sudograph::ic_cdk::export::Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").expect("should be able to decode"),


    //];

    //if  principals.contains(sudograph::ic_cdk::caller()) {
    //    panic!("Not authorized");
    //}

    return graphql_mutation(query, variables).await;
}