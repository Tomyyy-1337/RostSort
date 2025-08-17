rost::rost! { 
    nutze rand::zufall;
    nutze std::zeit::SystemZeit;
    
    fk einstieg() {
        lass anzahl = 100_000_000;
        lass liste = (0..anzahl)
            .zuordnen(|_| zufall::<u16>())
            .sammeln::<Vektor<_>>();

        lass änd liste_1 = liste.klonen();
        lass änd liste_2 = liste.klonen();

        lass start_zeit = SystemZeit::jetzt();
        lass ergebis = Sortierung::zähl_sortierung(&mut liste_1);
        lass dauer = start_zeit.vergangen().auspacken();

        entspreche ergebis {
            Gut(_) => ausgabe!("Zeit für Zähl-Sortierung: {:?}", dauer),
            Fehler(FehlerArt::TypZuGroß) => {
                ausgabe!("Der Typ ist zu groß für die Zähl-Sortierung!");
                zurückgebe;
            }
        }
    
        lass start_zeit = SystemZeit::jetzt();
        liste_2.sort_unstable();
        lass dauer = start_zeit.vergangen().auspacken();
        
        ausgabe!("Zeit für Standard-Sortierung: {:?}", dauer);

        behaupte_gleich!(liste_1, liste_2, "Die Listen sind nicht gleich!");
    }

    struktur Sortierung;

    umstz Sortierung{
        fk zähl_sortierung<T>(scheibe: &änd [T]) -> Ergebnis<(), FehlerArt>
        wo 
            T: Kopieren + Hinein<usize>
        {
            lass anzahl_stücke = größe_von::<T>() * 8;
            wenn anzahl_stücke > 32 {
                zurückgebe Fehler(FehlerArt::TypZuGroß);
            }

            lass anzahl_eimer = 1 << anzahl_stücke;
            lass änd eimer = vec![0; anzahl_eimer];

            für &wert in scheibe.wieder() {
                lass index: usize = wert.hinein();
                eimer[index] += 1;        
            }

            lass änd j = 0;
            für i in 0..anzahl_eimer {
                lass wert = gefährlich { std::zeiger::lese(&i als *konstante usize als *konstante T) };
                lass anzahl = eimer[i];
                scheibe[j..j+anzahl].füllen(wert);
                j += anzahl;                
            }    
            zurückgebe Gut(());
        }
    }

    aufzählung FehlerArt {
        TypZuGroß,
    }
}   
