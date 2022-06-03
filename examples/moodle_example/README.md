### Moodle Matcher / Course Organization

This is a small example that you might use to organize files downloaded from a moodle page. This example matches the page ID in the moodle URL and uses the course name as the folder name.

If it can't match it prompts a dialog for the user to pick a directory within that semester's directory.

It also takes advantage of hash storage to make sure that a decision about the file is remembered. This happens through `store_hash_location_map` being `true` and the `hash_storage` matcher coming first in the list.
