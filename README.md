
*   `--sort <cpu|memory>`, `-s <cpu|memory>`
    *   Sorts the process list by either CPU usage or memory usage.
    *   Default: `memory`.
*   `--mode <show|watch>`, `-m <show|watch>`
    *   `show`: Displays a single snapshot of the processes and exits.
    *   `watch`: Continuously refreshes the process list at a specified interval.
    *   Default: `show`.
*   `--count <NUMBER>`, `-c <NUMBER>`
    *   Specifies the number of processes to display.
    *   Default: `10`.
*   `--summary`, `-u`
    *   Displays a summary of system information (CPU, memory, load, uptime) before the process list.
*   `--kill <PID>`, `-k <PID>`
    *   Attempts to kill the process with the specified Process ID (PID).
    *   When this option is used, the tool will attempt to kill the process and then exit, ignoring other display-related options.
*   `--rate <SECONDS>`, `-r <SECONDS>`
    *   Sets the refresh rate in seconds for `watch` mode. Must be a positive number.
    *   Default: `1`.
