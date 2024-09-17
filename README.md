# CLI Organizer

CLI Organizer ist eine Rust-basierte Anwendung, die eine grafische Benutzeroberfläche (GUI) bereitstellt, um verschiedene CLI-Befehle zu durchsuchen und deren Manpages anzuzeigen. Die Anwendung nutzt das eframe-Framework für die GUI und syntect für die Syntaxhervorhebung der Manpages.

## Hauptfunktionen:
- **Befehlsdurchsuchung**: Durchsuchen Sie eine Liste von CLI-Befehlen und filtern Sie sie basierend auf einem Suchbegriff.
- **Manpage-Anzeige**: Zeigen Sie die Manpage des ausgewählten Befehls an.
- **Freitextsuche in Manpages**: Suchen Sie innerhalb der angezeigten Manpage nach einem bestimmten Text.
- **Scroll-Funktionen**: Scrollen Sie schnell zum Anfang oder Ende der Manpage.
- **Holen aller Anwendungen des Systems**: Die Anwendung verwendet den `compgen`-Befehl, um automatisch alle verfügbaren CLI-Befehle des Linux-Systems zu holen.

## Nutzung:
1. Starten Sie die Anwendung.
2. Geben Sie im linken Panel einen Suchbegriff ein, um die Liste der CLI-Befehle zu filtern.
3. Wählen Sie einen Befehl aus der gefilterten Liste aus, um dessen Manpage im rechten Panel anzuzeigen.
4. Verwenden Sie die Suchleiste über der Manpage, um innerhalb der Manpage nach einem bestimmten Text zu suchen.
5. Nutzen Sie die Schaltflächen "Scroll to Top" und "Scroll to Bottom", um schnell zum Anfang oder Ende der Manpage zu scrollen.

# Beschreibung auf Englisch

## CLI Organizer

CLI Organizer is a Rust-based application that provides a graphical user interface (GUI) to browse various CLI commands and display their manpages. The application uses the eframe framework for the GUI and syntect for syntax highlighting of the manpages.

## Main Features:
- **Command Search**: Browse and filter a list of CLI commands based on a search term.
- **Manpage Display**: Display the manpage of the selected command.
- **Free Text Search in Manpages**: Search within the displayed manpage for a specific text.
- **Scroll Functions**: Quickly scroll to the top or bottom of the manpage.
- **Fetch All System Applications**: The application uses the `compgen` command to automatically fetch all available CLI commands of the Linux system.

## Usage:
1. Start the application.
2. Enter a search term in the left panel to filter the list of CLI commands.
3. Select a command from the filtered list to display its manpage in the right panel.
4. Use the search bar above the manpage to search for a specific text within the manpage.
5. Use the "Scroll to Top" and "Scroll to Bottom" buttons to quickly scroll to the top or bottom of the manpage.