# File Input: native picker checkpoint

`src/file_input/` uses a native file input with FieldLabel. The browser owns the selection and form behavior; the callback hands Dioxus file handles to the caller without reading or uploading their contents. `accept` is a picker hint, not file validation.

Pinned Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `FileInput/FileInput.tsx` (`ae785ff55c0c509e94d3ba3ba47c8dee80ace777`), its spec (`79ac7548651cebfe90b09ee042eeee7291c1388f`), and the showcase (`f2477f4f8b237a288491075878ee365ca43514eb`). The adapted Upload file example keeps the label and width, uses the browser's filename and picker button, and reports only the selection count.

All five Rust checks and three Chrome suites passed. The focused Chrome suite used an offline text fixture to verify native label and description association, required validity, selection callback, form data, native reset, the multiple attribute, and disabled form exclusion. It did not automate the operating system picker. Dropzone, controlled selection and removal, validation limits, async status, remaining catalog examples, and visual parity remain open. The checklist remains open.
