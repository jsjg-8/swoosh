import os

# Define the path to the 'src' folder and the output markdown file
src_folder = "src"
output_file = "scripts_with_contents.md"


def list_scripts(src_folder):
    # Get all script files in the 'src' folder (filter .py, .js, .ts, etc.)
    script_files = []
    for root, dirs, files in os.walk(src_folder):
        for file in files:
            # Customize the extension filter as needed
            if file.endswith(".rs"):
                script_files.append(
                    os.path.relpath(os.path.join(root, file), start=src_folder)
                )
    return script_files


def write_to_md(script_files, output_file, src_folder):
    with open(output_file, "w", encoding="utf-8") as md_file:
        md_file.write("# List of Script Files and Their Contents\n\n")
        md_file.write(
            "The following are all the script files in the `src` folder along with their contents:\n\n"
        )

        for script in script_files:
            # Write script file name
            script_path = os.path.join(src_folder, script)
            md_file.write(f"## {script}\n\n")
            md_file.write("```rust\n")

            try:
                # Write script content
                with open(script_path, "r", encoding="utf-8") as f:
                    md_file.write(f.read())
            except Exception as e:
                md_file.write(f"Error reading file: {e}")

            md_file.write("\n```\n\n")


# Main execution
if __name__ == "__main__":
    if not os.path.exists(src_folder):
        print(f"Error: Folder '{src_folder}' does not exist.")
    else:
        script_files = list_scripts(src_folder)
        write_to_md(script_files, output_file, src_folder)
        print(
            f"Markdown file '{output_file}' has been generated with the list of scripts and their contents."
        )
