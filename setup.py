from setuptools import setup, find_packages

with open('README.md', 'r', encoding='utf-8') as f:
    readme = f.read()

with open('musicscraper/__init__.py', 'r', encoding='utf-8') as f:
    for line in f:
        if line.startswith('__version__'):
            version = eval(line.split('=')[1])

setup(
    name='musicscraper',
    version=version,
    author='TurtleP',
    author_email='jpostelnek@outlook.com',
    license='MIT',
    url='https://github.com/TurtleP/MusicScraper',
    python_requires='>=3.8.0',
    description='Music Metadata Scraper',
    long_description=readme,
    long_description_content_type='text/markdown',
    install_requires=["progress", "taglib"],
    packages=find_packages(),
    package_data={},
    entry_points={'console_scripts': ['musicscraper=musicscraper.__main__:main']},
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Operating System :: POSIX :: Linux'
    ]
)
