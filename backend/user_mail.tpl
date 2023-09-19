Hallo {{name}},

vielen Dank für die Meldung für {{club}} zum PSV Indoor Turnier 2024.

Folgene Schützen wurden eingetragen:

{{#each archers}}
Name: {{this.first_name}} {{this.last_name}}
Geburtsdatum: {{this.date_of_birth}}
Klasse: {{this.class}}
Scheibe: {{this.target_face}}

{{/each}}

Wir bitten um eine baldige Überweisung der Startgebühr.
Betrag: {{total_price}}
IBAN: DE12 4567 ...
Verwendungszweck: Indoor {{club}}

Viele Grüße und Alle ins Gold
Sportleitung der Bogenabteilung des PSV München