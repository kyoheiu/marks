# marks
Minimal online text editor.
Heavily WIP.

## features
- Everything is text: in `data` directory you store text files, and marks enables you to edit or delete, or add a new one.
- With '.md' extension, text is rendered as markdown file.
- But, no image rendering.
- No tags, no categories, no subdirectories.
- Search with `ripgrep` (supports regex pattern).
- Filter file names.
- Press <C-CR> in editable area to save.

## deploy
`sudo docker run -d -v /path/to/data:/marks/data --name marks -p 8080:8080 kyoheiudev/marks:0.2.3`
