from setuptools import find_packages, setup

with open('README.md', 'r', encoding='utf-8') as f:
    readme = f.read()

from tagscraper import __version__

setup(
    name='musicscraper',
    version=__version__,
    author='TurtleP',
    author_email='jpostelnek@outlook.com',
    license='MIT',
    url='https://github.com/TurtleP/tagscraper',
    python_requires='>=3.8.0',
    description='Music Metadata Scraper',
    long_description=readme,
    long_description_content_type='text/markdown',
    install_requires=["pytaglib"],
    packages=find_packages(),
    package_data={},
    entry_points={'console_scripts': [
        'tagscraper=tagscraper.__main__:main']},
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Operating System :: POSIX :: Linux'
    ]
)
