import config
import table
import os
from loguru import logger


def main():
    """Main function for the program
    """
    config_contents = config.load_config('/github/workspace')
    table_lines = table.generate_table(config_contents)
    with open('README.md') as readme_file:
        readme_orig_lines = readme_file.readlines()
    if '<!-- START OF PROFILE STACK, DO NOT REMOVE -->\n' not in readme_orig_lines:
        with open('README.md', 'a') as readme_file:
            readme_file.write(
                '\n<!-- START OF PROFILE STACK, DO NOT REMOVE -->\n')
            for table_line in table_lines:
                if table_line == '\n' or table_line.strip() == '':
                    readme_file.write(table_line)
                else:
                    readme_file.write(table_line + '\n')
            readme_file.write('<!-- END OF PROFILE STACK, DO NOT REMOVE -->\n')
            logger.success('Wrote table to README.md file')
    else:
        with open('README.md', 'w') as readme_file:
            readme_file.write('')
        with open('README.md', 'a') as readme_file:
            write_line = True
            for line in readme_orig_lines:
                if line == '<!-- END OF PROFILE STACK, DO NOT REMOVE -->\n':
                    readme_file.write('\n<!-- END OF PROFILE STACK, DO NOT REMOVE')
                    write_line = True
                if write_line:
                    if line == '\n' or line.strip() == '':
                        readme_file.write(line)
                    else:
                        readme_file.write(line + '\n')
                if line == '<!-- START OF PROFILE STACK, DO NOT REMOVE -->\n':
                    for table_line in table_lines:
                        readme_file.write(table_line + '\n')
                    write_line = False
                logger.success('Wrote table to README.md file')
    os.system('git config --global user.email "action@github.com"')
    os.system('git config --global user.name "Publishing Bot"')
    os.system('git add .')
    os.system('git commit -m "Update profile stack"')
    os.system('git push')
    logger.success('Pushed changes!')


if __name__ == "__main__":
    main()
