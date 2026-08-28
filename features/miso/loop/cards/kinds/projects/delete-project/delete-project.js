// /delete's two-tap, bound to the projects bin: the same three seconds, the
// same word, its own armed state. Absent /delete there is no bin to bind.
const feature_DeleteProject = typeof feature_Delete !== 'undefined'
  ? feature_Delete.make('projects_delete')
  : null;
