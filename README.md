# CLI Organizer

<p style="color: #2E8B57; font-size: 1.2em;">
<strong>CLI Organizer</strong> ist eine Rust-basierte Anwendung, die eine grafische Benutzeroberfläche (GUI) bereitstellt, um verschiedene CLI-Befehle zu durchsuchen und deren Manpages anzuzeigen. Die Anwendung nutzt das <strong>eframe</strong>-Framework für die GUI und <strong>syntect</strong> für die Syntaxhervorhebung der Manpages.
</p>

## <span style="color: #4682B4;">Hauptfunktionen:</span>
<ul>
  <li><strong>Befehlsdurchsuchung:</strong> Durchsuchen Sie eine Liste von CLI-Befehlen und filtern Sie sie basierend auf einem Suchbegriff.</li>
  <li><strong>Manpage-Anzeige:</strong> Zeigen Sie die Manpage des ausgewählten Befehls an.</li>
  <li><strong>Freitextsuche in Manpages:</strong> Suchen Sie innerhalb der angezeigten Manpage nach einem bestimmten Text.</li>
  <li><strong>Scroll-Funktionen:</strong> Scrollen Sie schnell zum Anfang oder Ende der Manpage.</li>
  <li><strong>Holen aller Anwendungen des Systems:</strong> Die Anwendung verwendet den <code>compgen</code>-Befehl, um automatisch alle verfügbaren CLI-Befehle des Linux-Systems zu holen.</li>
</ul>

## <span style="color: #4682B4;">Nutzung:</span>
<ol>
  <li><strong>Starten Sie die Anwendung.</strong></li>
  <li><strong>Geben Sie im linken Panel einen Suchbegriff ein, um die Liste der CLI-Befehle zu filtern.</strong></li>
  <li><strong>Wählen Sie einen Befehl aus der gefilterten Liste aus, um dessen Manpage im rechten Panel anzuzeigen.</strong></li>
  <li><strong>Verwenden Sie die Suchleiste über der Manpage, um innerhalb der Manpage nach einem bestimmten Text zu suchen.</strong></li>
  <li><strong>Nutzen Sie die Schaltflächen "Scroll to Top" und "Scroll to Bottom", um schnell zum Anfang oder Ende der Manpage zu scrollen.</strong></li>
</ol>

<hr style="border: 1px solid #4682B4;">

# Description in English

## <span style="color: #4682B4;">CLI Organizer</span>

<p style="color: #2E8B57; font-size: 1.2em;">
<strong>CLI Organizer</strong> is a Rust-based application that provides a graphical user interface (GUI) to browse various CLI commands and display their manpages. The application uses the <strong>eframe</strong> framework for the GUI and <strong>syntect</strong> for syntax highlighting of the manpages.
</p>

## <span style="color: #4682B4;">Main Features:</span>
<ul>
  <li><strong>Command Search:</strong> Browse and filter a list of CLI commands based on a search term.</li>
  <li><strong>Manpage Display:</strong> Display the manpage of the selected command.</li>
  <li><strong>Free Text Search in Manpages:</strong> Search within the displayed manpage for a specific text.</li>
  <li><strong>Scroll Functions:</strong> Quickly scroll to the top or bottom of the manpage.</li>
  <li><strong>Fetch All System Applications:</strong> The application uses the <code>compgen</code> command to automatically fetch all available CLI commands of the Linux system.</li>
</ul>

## <span style="color: #4682B4;">Usage:</span>
<ol>
  <li><strong>Start the application.</strong></li>
  <li><strong>Enter a search term in the left panel to filter the list of CLI commands.</strong></li>
  <li><strong>Select a command from the filtered list to display its manpage in the right panel.</strong></li>
  <li><strong>Use the search bar above the manpage to search for a specific text within the manpage.</strong></li>
  <li><strong>Use the "Scroll to Top" and "Scroll to Bottom" buttons to quickly scroll to the top or bottom of the manpage.</strong></li>
</ol>