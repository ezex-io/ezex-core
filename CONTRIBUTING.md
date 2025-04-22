# How to contribute

Thank you for considering to help out with the source code!
Please make sure you have read this document before start working on this project.

## Clone the repository

You need to clone this repository to get started:
```
git clone git@github.com:ezex-io/ezex-core.git
```

Please make sure you have defined a ssh keys before cloning the repository.

## Define a branch

When you want to make changes, you'd create a new branch from the HEAD of main branch.

```
git fetch
git checkout -b my-branch origin/main
```

## Testing coupe is a first class coupe

In our journey, testing coupe is the first class coupe.
Never submit a change without testing. Don't mess up  with the test code. *Test code is just as important  as production code*.
So write test before coding!

make sure your changes haven't broken anything:
```
cargo test
```

## Code convention

We have linting and formatting tools to check the code convention. Please make sure you are following the Rust code convention.
Format your change by running this command:
```
cargo fmt --all
cargo clippy --all-targets --all-features
```

## Merge request

Please never push code directly to the main branch. Try to push your changes to the branch of the same name on the remote:
```
git push origin HEAD
```
When the change is finished and all the tests are passed, you can send a merge request to the main branch.

Write some useful and relevant information about your change in the description field and also refer the ticket ID. Ticket ID should start with `CC-##` and it should be **uppercase**.




