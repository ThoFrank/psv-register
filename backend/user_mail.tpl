Hallo {{name}},

vielen Dank für die Meldung für {{club}} zum PSV Indoor Turnier 2024.

angegebener Kommentar:
{{comment}}

Folgene Schützen wurden eingetragen:

{{#each archers}}
Name: {{this.first_name}} {{this.last_name}}
Geburtsdatum: {{this.date_of_birth}}
Gruppe: {{this.session}}
Klasse: {{this.class}} ({{this.price}})
Scheibe: {{this.target}}

{{/each}}

Wir bitten um eine baldige Überweisung der Startgebühr.
Betrag: {{total_price}}
IBAN: DE12 4567 ...
Verwendungszweck: Indoor - {{club}}

Viele Grüße und Alle ins Gold
Sportleitung der Bogenabteilung des PSV München