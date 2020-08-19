K2ExtractRWD is a simple program to extract the content of the two RWD files
containing the game data and music for Kohan II: Kings of War. The author, Joe
Schlimgen, is releasing it as freeware; you can give away unmodified copies as
long as you include this readme file, but you can't sell it -- separately or as
part of a larger package.

Although every effort has been made to ensure that it works correctly, the
author cannot take any responsibility for any damage it does to your system.
Use at your own risk.

Any suggestions or bug reports can be sent to -removed email-

The program attempts to locate your game directory using the registry entries.
Failing that, you will have to locate the directory on your own. You can
extract the game data and/or the music. In either case, any files currently in
your Data sub-directory will be overwritten so be sure you save any of your
mods first.

==============================================================================

For those interested, here's enough of the format of the RWD file to extract
files.

header
    char[30]        unknown

file content

file directory
    int             number of files
    file entry      one for each file
        short       length of file name (in unicode characters)
        wchar_t[]   unicode file name (relative to Data directory)
        int         offset to file content from end of header
        int         unknown
        int         file length
        int         unknown
        int         unknown
        int         unknown

trailer
    char[280]       unknown
    int             length of file directory
    int             unknown

char is 8 bits; wchar_t is 16 bits.
int is 32 bits; short is 8 bits.

==============================================================================

Finally, some pseudo code to extract files.

// Locate and read the trailer

seek_from_end (sizeof (trailer));
read (trailer, sizeof (trailer));

// Locate file directory and read the number of entries

seek_from_end (sizeof (trailer) + trailer.directory_length);
read (file_count, sizeof (file_count));

// Extract the files

for (ix = 0; ix < file_count; ++ix)
    {
    // Read the file entry

    read (filename_length, sizeof (filename_length));
    read (filename, filename_length * sizeof (wchar_t));
    read (offset,  sizeof (offset));
    read (unknown, sizeof (unknown));
    read (length,  sizeof (length));
    read (unknown, sizeof (unknown));
    read (unknown, sizeof (unknown));
    read (unknown, sizeof (unknown));

    // Create directory structure (if necessary) and open output file

    create_directories (filename);
    create_file (filename);

    // Locate, read, and save file content
    // (current position must be saved/restored)

    pos = tell();

    seek (offset + sizeof (header));
    read (content, length);
    write (content, length);

    seek (pos);
    }

==============================================================================

