# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop
```

# Config (since 2.0)

Settings can be defined in Environment variables, in the conf.json or as flags.
Each variant overrides the previous one, in the order:

1. Default
2. conf.json
3. .env file
4. Environment
5. flag

One exception is the `conf` variable, which is read before, then the conf.json is read and all settings merged.

| Default     | conf.json       | Environment         | flag              | Description                                                                               |
|-------------|-----------------|---------------------|-------------------|-------------------------------------------------------------------------------------------|
| `conf.json` | -               | `FSM_CONF`          | `--conf`          | Specify location of Factorio Server Manager config file (relative to the server manager). |
| `./manager` | `manager_path`  | `FSM_MANAGER_PATH`  | `--manager_path`  | Specifiy the path the manager saves all factorio data.                                    |
| `./fsm.db`  | `database_file` | `FSM_DATABASE_FILE` | `--database_file` | Specify the path to the database file.                                                    |
