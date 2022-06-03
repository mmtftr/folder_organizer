const courseMap = {
  53218: "math",
  52723: "cmpe",
};

function moveToCourseAndShow(subdirectory, orig_filename) {
  return {
    action: "multiple",
    args: [
      {
        action: "move",
        args: "21-22/spring22/" + subdirectory + "/" + orig_filename,
      },
      { action: "show" },
    ],
  };
}

/**
 * Given file information, returns the action(s) to be executed
 * for that file.
 * 
 * @argument {FileInfo} file_info 
 * @returns {FileAction}
 */
function get_actions(file_info) {
  // As defined in FileExtras
  const froms = file_info.extras.xattr_apple_metadata_kmditemwherefroms;
  const orig_filename = file_info.name;

  // Handle different format course URLs.
  if (froms.some((url) => url.indexOf("notion") !== -1)) {
    return moveToCourseAndShow("eng", orig_filename);
  }

  // Match moodle page ID to course name.
  const courseRegex = new RegExp(
    // Put your school's moodle page regex here.
    "^https://moodle.yourschool.edu/course/view.php?id=(d+)$"
  );

  for (const from of froms) {
    const match = courseRegex.exec(from);
    if (match !== null) {
      if (courseMap[match[1]]) {
        return moveToCourseAndShow(courseMap[match[1]], orig_filename);
      }
    }
  }

  // Haven't matched anything, so fallthrough to another matcher.
  return { action: "fallthrough" };
}
