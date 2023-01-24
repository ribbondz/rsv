pub const COUNT_DESC: &str = "Count the number of lines. The command deals with raw bytes
and is therefore fast. When supplemented with a directory, the command count the number of
files in the directory.

Usage: 
  rsv.exe count [OPTIONS] <FILENAME>
  rsv count data.csv                 # data
  rsv count --no-header data.csv     # no header
  rsv count directory                # directory
  rsv count EXCEL.xlsx               # EXCEL file

Arguments:
  <FILENAME>  File to open

Options:
      --no-header      Whether the file has a header
  -S, --sheet <SHEET>  Get the nth worksheet of Excel file [default: 0]
  -h, --help           Print help information (use `--help` for more detail)
";

pub const HEADER_DESC: &str = "Show file headers. 

Usage: 
  rsv.exe headers [OPTIONS] <FILENAME>
  rsv headers data.csv
  rsv headers -s \\t data.csv
  rsv headers --sheet 0 data.xlsx
  rsv headers --sheet 1 data.xlsx

Arguments:
  <FILENAME>  File to open, e.g., CSV, TXT, and EXCEL

Options:
  -s, --sep <SEP>      Field separator [default: ,]
  -S, --sheet <SHEET>  Get the nth worksheet of EXCEL file [default: 0] 
  -h, --help           Print help information";

pub const HEAD_DESC: &str = "Show head n lines of file. When it is a CSV or TXT file, the result 
could be formatted as an aligned table by setting the --tabled flag. 
An EXCEL file is printed in default as an aligned table.

Usage: 
  rsv.exe head [OPTIONS] <FILENAME>
  rsv head data.csv                   # print as the file is
  rsv head --tabled data.csv          # tabled 
  rsv head -t data.csv                # tabled too
  rsv head -s \\t -t data.csv          # CSV file with a tab separator
  rsv head data.xlsx                  # first sheet of EXCEL file
  rsv head --sheet 1 data.xlsx        # second sheet

Arguments:
  <FILENAME>  File to open, e.g., CSV, TXT, EXCEL or OTHERS

Options:
  -s, --sep <SEP>      Separator [default: ,]
      --no-header      Whether the file has a header
  -n, --n <N>          Number of records to show [default: 10]
  -t, --tabled         Print as a table
  -S, --sheet <SHEET>  Get the nth worksheet of EXCEL file [default: 0]
  -h, --help           Print help information
";

pub const ESTIMATE_DESC: &str = "Fast estimate the number of lines. 
    
The command first read 20000 lines (except the header) from the CSV file, 
and then estimate average bytes of a line by dividing total bytes and
number of lines.

The total number of lines of CSV file is estimated according to file size
and average bytes per line.

The estimate is every fast.

Usage: 
  rsv.exe estimate <FILENAME>
  rsv.exe estimate data.csv
  rsv.exe estimate data.xlsx

Arguments:
  <FILENAME>  File to open

Options:
  -S, --sheet <SHEET>  Get the nth worksheet for an Excel file [default: 0]
  -h, --help           Print help information 
";

pub const CLEAN_DESC: &str = "Clean file with escape chars (e.g. \"). Other special strings
can also be cleaned. Do not support EXCEL files.

Usage: 
  rsv.exe clean [OPTIONS] <FILENAME>
  rsv clean data.csv                                 # default to clean escape char \"
  rsv clean -e \"content-to-delete\" data.csv          # clean str to empty
  rsv clean -o new-file.csv data.csv                 # save to new-file.csv

Arguments:
  <FILENAME>  File to open

Options:
  -o, --output <F>       Output file, default to current-file-cleaned.csv
  -e, --escape <ESCAPE>  Escape char to clean [default: \"]
  -h, --help             Print help information";

pub const FLATTEN_DESC: &str =
    "Prints flattened records to view records one by one. Records are separated
by \"#\", which could be changed with the --delimiter flag. The command is 
similar to \"xsv flatten\" and \"\\G\" command in mysql.

Usage: 
  rsv.exe flatten [OPTIONS] <FILENAME>
  rsv flatten data.csv                       # default to show first 5 records
  rsv flatten -n 50 data.csv                 # show 50 records
  rsv flatten --delimiter \"--\" data.csv      # change delimiter to anything
  rsv flatten data.xlsx                      # EXCEL, default to first sheet
  rsv flatten --sheet 1 data.xlsx            # EXCEL, second sheet

Arguments:
  <FILENAME>  File to open, CSV, TXT, or EXCEL

Options:
  -s, --sep <SEP>              Separator [default: ,]
      --no-header              Whether the file has a header
  -d, --delimiter <DELIMITER>  Line delimiter for printing [default: #]
  -n, --n <N>                  Number of records to show, n=-1 to show all [default: 5]
  -S, --sheet <SHEET>          Get the nth worksheet of EXCEL file [default: 0]
  -h, --help                   Print help information
";

pub const SLICE_DESC: &str = "Prints a slice of rows from CSV, TXT or EXCEL file. 
        
The range is [start, end).    
If the start is omitted , the slice starts from the first record of CSV.
If the end is omitted, the slice continuous to the last record of CSV.
    
A slice length can be also specified by the --len flag,
when the length is specified, the end index is ignored.
    
A single data record can by retrieved by the --index (shortened as -i) flag.
when -i is specified, other flags (including start, end, length) are all ignored.

Usage: 
  rsv slice [Options] <FILENAME>
  rsv slice -s 100 -e 150 data.csv           # set start and end
  rsv slice -s 100 -l 50 data.csv            # set start and length
  rsv slice -s 100 -l 50 --export data.csv   # export to data-slice.csv
  rsv slice -e 10 --export data.csv          # set end and export data
  rsv slice -i 9 data.csv                    # the 10th line sliced only
  rsv slice -i 9 data.xlsx                   # EXCEL file

Arguments:
  <FILENAME>  File to open, including CSV, TXT, and EXCEL

Options:
  -s, --start <START>    Start index of file [default: 0]
  -e, --end <END>        End index of file
  -l, --length <LENGTH>  Slice length
  -i, --index <INDEX>    Index for a single record
      --no-header        Whether the file has a header
  -E, --export           Export data to a current-file-slice.csv?
  -S, --sheet <SHEET>    Get the nth worksheet of EXCEL file [default: 0]
  -h, --help             Print help information
";

pub const FREQUENCY_DESC: &str =
    "Frequency table for one column or multiple columns. The columns are specified
by the --cols or -c flag. Column syntax's are either -c 0,1,2,5 or -c 0-2,5.

The command is performant because row of CSV or EXCEL are analyzed in parallel
(based on Rayon library) after their read in. 

The frequency table is shown in descending order by default, but it can be 
changed with the --ascending or -a flag.

The frequency table is printed to stdout by default, but it can be saved 
to a data-frequency.csv new file by the --export flag.

Usage: 
  rsv.exe frequency [OPTIONS] <FILENAME>
  rsv frequency -c 0 data.csv              # default to the first column, descending order
  rsv frequency -c 0,1,2,5 data.csv        # columns 0, 1, 2, and 5
  rsv frequency -c 0-2,5 data.csv          # same as above
  rsv frequency -c 0-2 --export data.csv   # export result to data-frequency.csv
  rsv frequency -n 10 data.csv             # keep top 10 frequent items
  rsv frequency -a 10 data.csv             # in ascending order
  rsv frequency data.xlsx                  # EXCEL file

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>      Separator [default: ,]
      --no-header      Whether the file has a header
  -c, --cols <COLS>    Columns to generate frequency table [default: 0]
  -a, --ascending      Ascending order or not
  -E, --export         Export result to a frequency.csv?
  -n, --n <N>          Top N to keep in frequency table [default: -1]
  -S, --sheet <SHEET>  Get the nth worksheet of EXCEL file [default: 0]
  -h, --help           Print help information

Column selection syntax:
-c 0,1,2,5   -->    cols [0,1,2,5]
-c 0-2,5     -->    same as cols [0,1,2,5]
";

pub const SPLIT_DESC: &str = "Split a big and unordered file into separate files: (1) sequentially;
(2) or based on a column value . In (2), Only one column can be specified
at a time, with the --col or -c flag. 

The output directory is created automatically within the current data
directory. Separate small files are named after unique column value. 

The command is performant because: (1) data is analyzed in chunks,
e.g., 50MB by default, other than by line by line, so that it avoid 
continuously opening and closing small files; (2) rows of CSV or EXCEL 
are analyzed in parallel (based on Rayon) after their read in.

The default mode is split based on values of first column.

Usage: 
  rsv.exe split [OPTIONS] <FILENAME>
  rsv split data.csv               # default to split based on first column 
  rsv split -c 1 data.csv          # second column to split
  rsv split -c 0 -s \\t data.csv    # first column, \\t separator
  rsv split data.xlsx              # EXCEL file
  rsv split --size 1000 data.xlsx  # sequential split, 1000 records in a file.

Arguments:
  <FILENAME>  File to open

Sequential Split options: 
  --size <SIZE>    Number of records to write in each separate file

Column-based Split Options: 
  -s, --sep <SEP>      Separator [default: ,]
      --no-header      Whether the file has a header
  -c, --col <COL>      Column to split upon [default: 0]
  -S, --sheet <SHEET>  Get the nth worksheet of EXCEL file [default: 0]
      --size <SIZE>    Number of records to write in each separate file
  -h, --help           Print help information
";

pub const SELECT_DESC: &str = "Select rows and columns by filter. Row and column filter syntaxes
are listed as below. Output can be exported with the --export flag.

Usage: 
  rsv.exe select [OPTIONS] <FILENAME>
  rsv select -f 0=a,b,c data.csv      # first column has values of a, b, or c
  rsv select -f \"0N>10&1=c\" data.csv  # first column > 10 numerically, AND the second column equals c
  rsv select -f 0!= --export data.csv # export result, in which the first column is non-empty
  rsv select -f 0=a,b data.xlsx       # apply to EXCEL file

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>        Separator [default: ,]
      --no-header        Whether the file has a header
  -c, --cols <COLS>      Columns to select, see column select syntax below; Default to select ALL
  -f, --filter <FILTER>  Row filter, see row filter syntax below; Default to NONE
  -E, --export           Export results to a file named current-file-selected.csv?
  -S, --sheet <SHEET>    Get the nth worksheet of EXCEL file [default: 0]
  -h, --help             Print help information

Filter syntax, support =, !=, >, >=, <, <= and &:
-f 0=a,b,c         -->  first column is a, b, or c
-f 0N=1,2          -->  first column numerically equals to 1 or 2
-f 0!=             -->  first column is not empty
-f 0>=2022-01-21   -->  first column equal to or bigger than 2022-01-21, lexicographically
-f 0N>10           -->  first column > 10 numerically
-f 0N>10&2=pattern -->  first column > 10 numerically, AND the third column equals to <pattern>

Column selection syntax:
-c 0,1,2,5         -->  cols [0,1,2,5]
-c 0-2,5           -->  same as cols [0,1,2,5]
";

pub const STATS_DESC: &str = "Statistics for every column, including min, max, mean, unique, null.
Within the command, every column is regarded as either an Int, Float or String.
When the column is String, min, max, mean are ignored. When the column is Float,
the unique stat is ignored.

The command process data in parallel, so that it is fast.

Usage: 
  rsv stats [OPTIONS] <FILENAME>
  rsv stats data.csv                       # all columns
  rsv stats -c 0,1 data.csv                # first two columns
  rsv stats -c 0,1 --export data.csv       # export statistics to data-stats.csv
  rsv stats -c 0,1 --export data.xlsx      # EXCEL file

Arguments:
  <FILENAME>  File to open, including CSV, TXT, and EXCEL

Options:
  -s, --sep <SEP>      Separator [default: ,]
      --no-header      Whether the file has a header
  -c, --cols <COLS>    Columns to generate statistics, column syntax 0,1,3 or 0-4, including 4; Default to select all
  -E, --export         Export results to a file named current-file-selected.csv?
  -S, --sheet <SHEET>  Get the nth worksheet of EXCEL file [default: 0]
  -h, --help           Print help information
";

pub const EXCEL2CSV_DESC: &str = "Convert EXCEL to CSV. All format information will be lost.

Usage: 
  rsv.exe excel2csv [OPTIONS] <FILENAME>
  rsv excel2csv data.xlsx              # default to first sheet
  rsv excel2csv --sheet 1 data.xlsx    # second sheet

Arguments:
  <FILENAME>  File to open

Options:
  -S, --sheet <SHEET>  Get the nth worksheet of EXCEL file [default: 0]
  -s, --sep <SEP>      Separator [default: ,]
  -h, --help           Print help information
";

pub const TABLE_DESC: &str = "Show data in an aligned table.

Usage: 
  rsv.exe table [OPTIONS]
  rsv head data.csv | rsv table                # convert result to an aligned table
  rsv slice -s 10 -e 15 data.csv | rsv table   # convert result to an aligned table

Options:
  -s, --sep <SEP>  Separator [default: ,]
  -h, --help       Print help information
";

pub const SEARCH_DESC: &str = "Search with regexes. Regex syntax is to be found at: https://docs.rs/regex/latest/regex/#syntax. 
The command reads file in chunks and processes a chunk in parallel based on Rayon.

Usage: 
  rsv.exe search [OPTIONS] <PATTERN> <FILENAME>
  rsv search PATTERN data.csv                     # regex search a PATTERN
  rsv search \"^\\d{4}-\\d{2}-\\d{2}$\" data.csv       # regex search dates
  rsv search --export PATTERN data.csv            # export result
  rsv search PATTERN data.xlsx                    # search EXCEL file

Arguments:
  <PATTERN>   Regex pattern to search
  <FILENAME>  File to open

Options:
      --no-header      Whether the file has a header
  -S, --sheet <SHEET>  Get the nth worksheet of EXCEL file [default: 0]
  -E, --export         Export to a file named current-file-searched.csv?
  -h, --help           Print help information
";

pub const SORT_DESC: &str =
    "Sort data by column(s). The sort is performed in-memory, so that it cannot
support large data files. The command supports sorting for at most two columns. 

The default is ascending sort. Descending sort can be specified with the 
-c 0D flag, where D stands for Descending Sort.  

The default is string sorting. Numeric sorting can be specified with the
-c 0N flag, where N stands for Numeric Sorting.

D (descending) and N (numeric) can be placed in arbitrary order, e.g., 
-c 0DN or -c 0ND.

Usage: 
  rsv sort [OPTIONS] [FILENAME]
  rsv sort -c 0 data.csv        # default to sort first column in ascending
  rsv sort -c 0D data.csv       # descending sort
  rsv sort -c 0DN data.csv      # sort as numeric values
  rsv sort -c 0DN,2N data.csv   # sort two columns
  rsv sort -E data.csv          # export result
  rsv sort data.xlsx            # sort EXCEL file

Arguments:
  [FILENAME]  File to open

Options:
  -s, --sep <SEP>      Separator [default: ,]
      --no-header      Whether the file has a header
  -c, --cols <COLS>    Columns to sort by, e.g., -c 0, -c 0N, -c 0ND [default: 0]
  -S, --sheet <SHEET>  Get the nth worksheet of EXCEL file [default: 0]
  -E, --export         Export to a file named current-file-sorted.csv?
  -h, --help           Print help
";
