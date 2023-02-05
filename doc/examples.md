# Examples

## Only showing content without using editor

Only showing content of yearly journal for year 2022 without opening it with the editor.
```text
daily_ruster_man year-edit --show-only 2022
```

## Specifying editor

Opening journal for tomorrow with the editor nvim
```text
daily_ruster_man e --editor nvim 1
```

## Deletion of journals

Deletes daily journal from yesterday
```text
daily_ruster_man d -1
```

Deletes monthly journal of august of the current year
```text
daily_ruster_man md 8
```

Deletes monthly journal of august of the in the year 2022.
It also will not safety prompt to confirm before deletion due to the argument 
```text
daily_ruster_man md --skip-confirmation 8 2022
```

Delete journal for year 1998
```text
daily_ruster_man yd 1998 
```
