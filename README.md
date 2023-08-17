# marks
Git-powered self-hostable online text editor.

## features
- Everything is text: In `data` directory you store text files, and marks enables you to edit or delete them, or add a new one.
- On save, the change will be automatically added and commited by git.
- No fancy editing feature such as WYSIWYG.
- No tags, no categories, no subdirectories.
- Texts with `.md` extension are converted to html in the view mode.
- Search with `ripgrep` (supports regex pattern).
- Filter file name.
- Lightweight: Compressed docker image size is ~50MB.

## deploy

1. If not initialized, `git init` in your `data` directory.
   To commit on save, add `user.name` and `user.config` to `data/.git/config` like this:
```
[user]
    name = Kyohei Uto
    email = "im@kyoheiu.dev"
```

2. Use `docker compose up -d` with i.e. the following `docker-compose.yml`:

```
version: '3.3'
services:
    marks:
        user: '1000:1000' # must be the same as the user who executed `git.init`.
        environment:
            - MARKS_GIT_USER="marks"  # default to 'marks'. Used for the signature when commiting.
            - MARKS_GIT_EMAIL="im@kyoheiu.dev" # default to 'git@example.com'
        volumes:
            - './data:/marks/data:rw'
            - '/etc/passwd:/etc/passwd:ro'
            - '/etc/group:/etc/group:ro'
        ports:
            - '8080:8080'
        container_name: marks
        image: 'kyoheiudev/marks:0.4.0'
        logging:
          driver: json-file
          options:
            max-size: 1m
            max-file: '3'
```

## tech stack
- frontend
  - svelte
- backend
  - rust(axum)
  - git2
