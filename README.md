# marks
Minimal self-hostable online text editor.

## features
- Everything is text: In `data` directory you store text files, and marks enables you to edit or delete them, or add a new one.
- No fancy editing feature such as WYSIWYG.
- No tags, no categories, no subdirectories.
- Texts with `.md` extension are converted to html in the view mode.
- Search with `ripgrep` (supports regex pattern).
- Filter file name.
- Auto save. You can also manually do it by pressing `<C-CR>` in editable areas.
- Lightweight: Compressed docker image size is ~50MB.

## deploy
`sudo docker run -d -v /path/to/data:/marks/data --name marks -p 8080:8080 kyoheiudev/marks:0.3.0`
