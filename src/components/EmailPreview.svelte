<script lang="ts">
    export var new_email: boolean = false;
    export var from: String = "Giesel, Jürgen";
    export var subject: String = "(UE) Programmierung (Übung - Tutorium): Selbstlernmodule \"Python4Java\" und \"C- und Shell-Programmierung\"";
    export var body: String = "Liebe Studierende, in den Semesterferien (vor Besuch der Vorlesungen \"Datenstrukturen und Algorithmen\" bzw. \"Betriebssysteme und Systemsoftware\") sollten Sie zwei Selbstlernmodule absolvieren. Diese bauen auf dem Stoff in der \"Programmierung\" auf und vermitteln Ihnen die Grundlagen in Python bzw. C. Hier sind zwei kurze Texte mit dem jeweiligen Moodle-Link für die beiden Module:";
    export var time: String = "2 min ago";
</script>


<div class="w-full border-b p-1 text-xs flex justify-between gap-1">
    {#if new_email}
        <div class="h-full pt-1">
            <div class="w-3 h-3 bg-blue-500 block rounded-full max-w-4" />
        </div>
    {/if}
    <div class="flex-grow">
        <h2 class="font-bold">{from}</h2>
        <h3 class="font-semibold overflow-ellipsis line-clamp-1">
            {subject}
        </h3>
        <p class="text-muted-foreground overflow-ellipsis line-clamp-1">
            {body}
        </p>
    </div>
    <div>
        <p class="text-muted-foreground w-max">{time}</p>
    </div>
</div>
