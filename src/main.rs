rost::rost! { 
    fk einstieg() {
        lass anzahl = 1_000_000;

        lass liste = (0..anzahl)
            .zuordnen(|_| rand::zufall::<u16>())
            .sammeln::<Vektor<_>>();

        lass änd liste_1 = liste.klonen();

        lass start_zeit = std::time::SystemTime::jetzt();

        lass ergebis = Sortierung::zähl_sortierung(&mut liste_1);

        lass dauer = start_zeit.vergangen().auspacken();

        entspreche ergebis {
            Gut(_) => ausgabe!("Zähl-Sortierung erfolgreich!"),
            Fehler(FehlerArt::TypeTooLarge) => {
                ausgabe!("Der Typ ist zu groß für die Zähl-Sortierung!");
                zurückgebe;
            }
        }
        
        ausgabe!("Zeit für Zähl-Sortierung: {:?}", dauer);

        lass änd liste_2 = liste.klonen();

        lass start_zeit = std::time::SystemTime::jetzt();

        liste_2.sort_unstable();

        lass dauer = start_zeit.vergangen().auspacken();
        ausgabe!("Zeit für Standard-Sortierung: {:?}", dauer);

        behaupte_gleich!(liste_1, liste_2, "Die Listen sind nicht gleich!");
    }

    

    struktur Sortierung;

    umstz Sortierung{
        fk zähl_sortierung<T>(scheibe: &änd [T]) -> Ergebnis<(), FehlerArt>
        wo 
            T: PartialOrdnung + Kopieren
        {
            lass anzahl_stücke = std::mem::größe_von::<T>() * 8;

            wenn anzahl_stücke > 32 {
                zurückgebe Fehler(FehlerArt::TypeTooLarge);
            }

            lass anzahl_eimer = 1 << anzahl_stücke;

            lass änd eimer = vec![0; anzahl_eimer];

            für wert in scheibe.wieder() {
                gefährlich {
                    let byte_liste = std::slice::aus_rohen_stücken(
                        (wert als *konstante T) als *konstante u8,
                        std::mem::größe_von::<T>(),
                    );
                    let index = byte_liste.wieder()
                        .nehme(8)
                        .drehe()
                        .falte(0, |acc, &byte| {
                            (acc << 8) | byte als usize
                        });
                    eimer[index] += 1;        
                }
            }

            lass mut j = 0;
            für i in 0..anzahl_eimer {
                lass wert = gefährlich { std::ptr::lese(&i als *konstante usize als *konstante T) };
                lass anzahl = eimer[i];
                scheibe[j..j+anzahl].füllen(wert);
                j += anzahl;                
            }    
            zurückgebe Gut(());
        }
    }

    aufzählung FehlerArt {
        TypeTooLarge,
    }
}   
