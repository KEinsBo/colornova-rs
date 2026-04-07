use openrgb2::{Color, OpenRgbClient, OpenRgbResult};



pub async fn set_all_leds(controller_index: usize, color: Color) -> OpenRgbResult<()> {
    let client = OpenRgbClient::connect().await;

    match client {
        Ok(client) => {
            let controller = match client.get_controller(controller_index).await {
                Ok(controller) => controller,
                Err(e) => {
                    eprintln!("Fehler beim Abrufen des Controllers: {}", e);
                    return Ok(());
                }
            };

            for _ in 0..3 { 
                let cmd = controller.cmd_with_leds(|_led| color);
                
                if let Err(e) = cmd.execute().await {
                    eprintln!("Fehler beim Setzen der LED-Farbe: {}", e);
                    continue;                 }

                break;             }
        }
        Err(e) => {
            eprintln!("Fehler bei der Verbindung mit dem OpenRGB-Client: {}", e);
        }
    }
    
    Ok(())
}
// Bessere funktion die nicht jedes mal neu verbindet
pub async fn set_all_leds_client(client: &OpenRgbClient, controller_index: usize, color: Color) -> OpenRgbResult<()> {
    let controller = match client.get_controller(controller_index).await {
        Ok(controller) => controller,
        Err(e) => {
            eprintln!("Fehler beim Abrufen des Controllers: {}", e);
            return Ok(());
        }
    };

    for _ in 0..3 {
        let cmd = controller.cmd_with_leds(|_led| color);

        if let Err(e) = cmd.execute().await {
            eprintln!("Fehler beim Setzen der LED-Farbe: {}", e);
            continue;
        }

        break;
    }

    Ok(())
}
