# Deployment


Before deploying the docker, a few things need to happen in order to finally release
and deploy the service.

## Updating Changelog

First make sure the service you are going to deploy has updated changelog.
You can find the `CHANGELOG.md` file inside the service's root directory.

## Tagging

To release a service, create a tag with this format:
```
<service_name>/v<version>-<beta|stable>
```

- `service_name` should be same as the [docker](./container/README.md) name.
- `version` should be same as the crate version.

Now you can create a tag and push it into git repository:

```
git tag -s -a <service-name>/v1.x.0-beta -m "<Service-Name> Version 1.x.0-beta"
git push origin <service-name>/v1.x.0-beta
```

## Deployment

Pushing a tag into gitlab repository will automatically
build the docker and push it to the docker repository.

## Bumping version

After deployment the version should be updated.
Update the version inside `Cargo.toml` file by increasing the minor version of the crate.
For example from `1.0.0` to `1.1.0`

Create a commit and push it to main branch:
```
git commit -m "Bumping <service-name> version to 1.x.0"
git push origin HEAD
```

Please make sure you update the [README](./README.md) file as well.
