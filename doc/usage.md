# Usage

## Configuration

This application can be altered via option in a configuration file named config.toml.
By default the app will look at the typical os location for app data. 

- For linux is usually ~/.conf/daily_ruster_man/

You can also provide the location via a command line option `--config-path` 
or the environment variable `CONFIG_PATH` instead. The path can be relative or absolute.

See the example [config.toml]. You can use this file as a starting point. 
it provides all options ready out comment them.

## Templates

This application allows you to create template files which are used when a journal is created.
The content of the template file is inserted for the journal 
before the journal is opened by the chosen editor
Which template file is used for what journal type (daily, monthly or yearly) can specified
in the configuration file (config.toml).
See the [example template file] how you can write a placeholder

## Placeholders

Templates can be augmented via placeholders. 
Each placeholder has a key and a value. The key is replaced with the respective value in template.
If the value should be treated as command then the field called is_command = true must be given 
to the key value pair. See the example [config.toml] how to provide placeholders.
Values can also be given as command which are executed as you have entered it in terminal.
The output of the command is then used as replacement for the key.

[config.toml]:config.toml
[example template file]:example.template
