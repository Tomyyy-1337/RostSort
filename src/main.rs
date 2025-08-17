rost::rost! { 
    fk einstieg() {
        lass anzahl = 1_000_000;

        lass liste = (0..anzahl)
            .zuordnen(|_| rand::random::<u16>())
            .sammeln::<Vektor<_>>();

        lass änd liste_1 = liste.clone();

        lass start_zeit = std::time::SystemTime::now();

        lass ergebis = Sortierung::zähl_sortierung(&mut liste_1);

        lass dauer = start_zeit.elapsed().unwrap();

        entspreche ergebis {
            Gut(_) => ausgabe!("Zähl-Sortierung erfolgreich!"),
            Fehler(FehlerArt::TypeTooLarge) => {
                ausgabe!("Der Typ ist zu groß für die Zähl-Sortierung!");
                zurückgebe;
            }
        }
        
        ausgabe!("Zeit für Zähl-Sortierung: {:?}", dauer);

        lass änd liste_2 = liste.clone();

        lass start_zeit = std::time::SystemTime::now();

        liste_2.sort_unstable();

        lass dauer = start_zeit.elapsed().unwrap();
        ausgabe!("Zeit für Standard-Sortierung: {:?}", dauer);

        behaupte_gleich!(liste_1, liste_2, "Die Listen sind nicht gleich!");
    }

    

    struktur Sortierung;

    umstz Sortierung{
        fk zähl_sortierung<T>(scheibe: &änd [T]) -> Ergebnis<(), FehlerArt>
        wo 
            T: PartialOrdnung 
        {
            lass anzahl_stücke = std::mem::size_of::<T>() * 8;

            wenn anzahl_stücke > 32 {
                zurückgebe Fehler(FehlerArt::TypeTooLarge);
            }

            lass anzahl_eimer = 1 << anzahl_stücke;

            lass änd eimer = vec![0; anzahl_eimer];

            für wert in scheibe.wieder() {
                gefährlich {
                    let arr = std::slice::from_raw_parts(
                        (wert als *konstante T) als *konstante u8,
                        std::mem::size_of::<T>(),
                    );
                    let index = arr.wieder()
                        .nehme(8)
                        .drehe()
                        .falte(0, |acc, &byte| {
                            (acc << 8) | byte als usize
                        });
                    eimer[index] += 1;        
                }
            }

            lass änd j = 0;
            für i in 0..anzahl_eimer {
                für _ in 0..eimer[i] {
                    gefährlich {
                        lass wert = std::ptr::read(&i als *konstante usize als *konstante T);
                        scheibe[j] = wert;
                        j += 1;
                    }
                }
            }    
            zurückgebe Gut(());
        }
    }

    aufzählung FehlerArt {
        TypeTooLarge,
    }
}   
