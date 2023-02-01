# Todos for this branch

- Create api which tries only once to get a option from cli, env and (config) 
available from start of the app. Need macro for this probably provided by a AppContext.
- Templates should all be able to be given via cli, env and config


## Thoughts

- Edit by date, month or year should be bundeled with new EditArgs into a new type like EditByDate
- EditArgs: Args for edit context like template path, no-template ... 
- This new edit types should provide the journal name, and choose if template path is for month, day 
or year via a trait. This will also free type like DailyNames from loading template path from config.
- AppOption should contain config as once cell for lazy loading of editor to use.

