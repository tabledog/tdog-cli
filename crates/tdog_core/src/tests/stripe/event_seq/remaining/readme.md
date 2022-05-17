- This dir acts like a to-do list to make it easier to understand which objects:
    - Need to be added to the download process.
    - Need to added to the apply_events process.
    

- It should prevent accidentally missing a type in either the download or the events.
    - Rust `match` blocks are complete, but only work for `UniNotificationEventDataObject` (it cannot ensure that the downloads are symmetric with the events).

- The download/apply_events processes should be mostly symmetric.
    - E.g. An object that is in the download should also get event updates.
    - These two have different functions because they come from 2 different Stripe API interfaces (direct download, event list).
    - Rust tests are good for splitting up tests, but have no feature to re-join all the results to get a complete picture.
        - As a result, grepping the source code is used to get a complete list of object types.

 
- List comes from Rust enum for `UniNotificationEventDataObject` (which in turn comes from the Open API spec). 
    - Converted to snake case with `https://textedit.tools/snakecase`.
    
- Editor based diff used to see remaining/to do.
    - Usage: `./ls-event-types.sh > event-types.txt`
    
    
- Assumptions:
    - All types that matter will have both download and events.
        - Is it possible some types only have a download interface (e.g. balance_transactions may need to be polled as they have no events).
        
        
        
# Getting the list if Sigma tables.

`copy($(`.db-SchemaBrowser-tableName`).toArray().map(x => x.innerHTML))`

- The Github list is an older list, missing payment intents.