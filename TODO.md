# Todos for this branch

- Switch from lazy static to once_cell crate
- Create api which tries only once to get a option from cli, env and (config) 
available from start of the app. Need macro for this probably provided by a AppContext.
- Refacture: Move loading config file outside from the DailyNames, MonthlyNames and YearlyNames
- Templates should all be able to be given via cli, env and config

