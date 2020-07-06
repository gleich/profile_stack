import config
import table
from loguru import logger


def main():
    """Main function for the program
    """
    config_contents = config.load_config('/github/workspace')
    table_lines = table.generate_table(config_contents)
    with open('README.md') as readme_file:
        readme_orig_lines = readme_file.readlines()
    if '<!-- START OF PROFILE STACK, DO NOT REMOVE -->' not in readme_orig_lines:
        with open('README.md', 'a') as readme_file:
            readme_file.write(
                '\n<!-- START OF PROFILE STACK, DO NOT REMOVE -->')
            readme_file.writelines(table_lines)
            readme_file.write('<!-- END OF PROFILE STACK, DO NOT REMOVE -->')
            logger.success('Wrote table to README.md file')
    else:
        with open('README.md', 'w') as readme_file:
            readme_file.write('')
        with open('README.md', 'a') as readme_file:
            write_line = True
            for line in readme_orig_lines:
                if line == '<!-- END OF PROFILE STACK, DO NOT REMOVE -->':
                    write_line = True
                if write_line:
                    readme_file.write(line)
                if line == '<!-- START OF PROFILE STACK, DO NOT REMOVE -->':
                    readme_file.writelines(table_lines)
                    write_line = False
                logger.success('Wrote table to README.md file')


if __name__ == "__main__":
    main()
