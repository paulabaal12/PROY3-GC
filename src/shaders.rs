use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use std::f32;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use crate::CelestialBody;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  match uniforms.current_body {
      CelestialBody::Sun => sun_shader(fragment, uniforms),
      CelestialBody::RockyPlanet => rocky_planet_shader(fragment, uniforms),
      CelestialBody::GasGiant => gas_giant_shader(fragment, uniforms),
      CelestialBody::CloudyPlanet => cloudy_planet_shader(fragment, uniforms),
      CelestialBody::RingedPlanet => ring_shader(fragment, uniforms),
      CelestialBody::IcePlanet => ice_planet_shader(fragment, uniforms),
      CelestialBody::ColorPlanet => colorful_planet_shader(fragment, uniforms),
      CelestialBody::Moon => moon_shader(fragment, uniforms),
      CelestialBody::OceanPlanet => ocean_planet_shader(fragment, uniforms),
      CelestialBody::AuroraPlanet => aurora_planet_shader(fragment, uniforms),
      CelestialBody::NaturePlanet => nature_planet_shader(fragment, uniforms),
  }
}

fn colorful_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.01;

    let color1 = Color::new(245, 56, 121);   
    let color2 = Color::new(245, 140, 105); 
    let color3 = Color::new(245, 115, 105); 
    let color4 = Color::new(245, 105, 238); 
    let color5 = Color::new(245, 159, 95);  
    let color6 = Color::new(245, 168, 162); 

    let ring1_color = Color::new(245, 7, 123); 
    let ring2_color = Color::new(245, 166, 195);  

    let curve_pattern = uniforms.noise.get_noise_3d(
        position.x * 5.0 + time * 1.5,
        position.y * 5.0,
        position.z * 5.0
    ).sin() * 0.5 + 0.5;

    let wave_pattern = (position.x * 15.0 + position.y * 15.0 + time).sin() * 0.5 + 0.5;
    
    let mut final_color = color1.lerp(&color2, curve_pattern);
    final_color = final_color.lerp(&color3, wave_pattern * 0.7);

    if curve_pattern > 0.6 {
        final_color = final_color.lerp(&color4, curve_pattern - 0.3);
    } else if wave_pattern > 0.5 {
        final_color = final_color.lerp(&color5, wave_pattern - 0.3);
    }

    let ring_pattern = uniforms.noise.get_noise_3d(
        position.x * 200.0 + time,
        position.y * 200.0,
        position.z * 200.0
    ).abs();

    if ring_pattern > 0.5 {
        final_color = final_color.lerp(&ring1_color, ring_pattern - 0.5);
    } else {
        final_color = final_color.lerp(&ring2_color, 0.5 - ring_pattern);
    }

    final_color * fragment.intensity
}

fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.01;

  let core_color = Color::new(255, 200, 0);      
  let corona_color = Color::new(255, 100, 0);    
  
  let plasma1 = uniforms.noise.get_noise_3d(
      position.x * 50.0 + time,
      position.y * 50.0,
      time * 2.0
  );
  
  let plasma2 = uniforms.noise.get_noise_3d(
      position.x * 30.0 - time,
      position.y * 30.0,
      time
  );
  
  let corona = uniforms.noise.get_noise_3d(
      position.x * 10.0,
      position.y * 10.0,
      time * 0.5
  ).abs();
  
  let combined_noise = (plasma1 + plasma2) * 0.5;
  let final_color = core_color.lerp(&corona_color, combined_noise.abs());

  let brightness = 1.0 + corona * 0.5;
  
  final_color * brightness * fragment.intensity
}

fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;

  let desert_color = Color::new(180, 80, 20);     
  let crater_color = Color::new(120, 50, 10);     
  let highland_color = Color::new(200, 100, 30);  
  
  let terrain = uniforms.noise.get_noise_3d(
      position.x * 100.0,
      position.y * 100.0,
      position.z * 100.0
  );
  
  let craters = uniforms.noise.get_noise_3d(
      position.x * 200.0 + 1000.0,
      position.y * 200.0 + 1000.0,
      position.z * 200.0
  ).abs();
  
  let dust = uniforms.noise.get_noise_3d(
      position.x * 50.0 + time,
      position.y * 50.0,
      position.z * 50.0
  );
  
  let mut final_color = desert_color;
  if craters > 0.7 {
      final_color = crater_color;
  } else if terrain > 0.3 {
      final_color = highland_color;
  }
  
  let dust_color = Color::new(200, 150, 100);
  final_color = final_color.lerp(&dust_color, dust.abs() * 0.3);
  
  final_color * fragment.intensity
}

fn cloudy_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.01;

  let surface_color = Color::new(30, 100, 200);  
  let land_color = Color::new(50, 120, 50);      
  let cloud_color = Color::new(255, 255, 255);   
  
  let surface = uniforms.noise.get_noise_2d(
      position.x * 100.0,
      position.y * 100.0
  );
  
  let clouds = uniforms.noise.get_noise_3d(
      position.x * 50.0 + time,
      position.y * 50.0 + time * 0.5,
      time
  );
  
  let base_color = if surface > 0.2 {
      land_color
  } else {
      surface_color
  };
  
  let final_color = if clouds > 0.3 {
      base_color.lerp(&cloud_color, (clouds - 0.3) * 2.0)
  } else {
      base_color
  };
  
  final_color * fragment.intensity
}

fn ring_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;
  
  let ring1_color = Color::new(180, 150, 120);  
  let ring2_color = Color::new(100, 80, 60);  
  
  let ring_pattern = uniforms.noise.get_noise_3d(
      position.x * 200.0 + time,
      position.y * 200.0,
      position.z * 200.0
  );
  
  let density = uniforms.noise.get_noise_2d(
      position.x * 100.0,
      position.y * 100.0
  );
  
  let final_color = if ring_pattern > 0.0 {
      ring1_color.lerp(&ring2_color, density.abs())
  } else {
      ring2_color
  };
  
  let alpha = (density.abs() * 0.5 + 0.5) * fragment.intensity;
  final_color * alpha
}

fn ice_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.002;

    // Paleta de colores expandida para efectos de hielo
    let ice_color = Color::new(220, 240, 255);        // Hielo superficial
    let deep_ice_color = Color::new(120, 180, 255);   // Hielo profundo
    let crack_color = Color::new(80, 130, 255);       // Grietas profundas
    let crystal_glow = Color::new(230, 255, 255);     // Brillo cristalino
    let aurora_ice = Color::new(160, 255, 220);       // Hielo con aurora
    let deep_blue = Color::new(40, 100, 255);         // Azul profundo
    let frost_white = Color::new(255, 255, 255);      // Escarcha brillante
    let twilight_ice = Color::new(180, 200, 255);     // Hielo crepuscular

    // Capas de hielo con variación temporal
    let ice_base = uniforms.noise.get_noise_3d(
        position.x * 80.0 + time * 0.1,
        position.y * 80.0,
        position.z * 80.0
    ).abs();

    let ice_detail = uniforms.noise.get_noise_3d(
        position.x * 150.0 + time * 0.2,
        position.y * 150.0,
        position.z * 150.0
    ).abs();

    // Sistema de grietas dinámicas
    let cracks_primary = uniforms.noise.get_noise_3d(
        position.x * 120.0 + time * 0.5,
        position.y * 120.0,
        position.z * 120.0
    ).abs();

    let cracks_secondary = uniforms.noise.get_noise_3d(
        position.x * 180.0 - time * 0.3,
        position.y * 180.0,
        position.z * 180.0
    ).abs();

    // Cristales de hielo multicapa
    let crystals_large = uniforms.noise.get_noise_3d(
        position.x * 200.0 + time * 0.1,
        position.y * 200.0,
        position.z * 200.0
    ).abs();

    let crystals_small = uniforms.noise.get_noise_3d(
        position.x * 300.0 + time * 0.2,
        position.y * 300.0,
        position.z * 300.0
    ).abs();

    // Efecto de aurora en el hielo
    let aurora_effect = ((position.x * 3.0 + time).sin() * 
                        (position.y * 3.0 + time * 0.7).cos() * 
                        (position.z * 3.0 + time * 0.5).sin()).abs();

    // Patrón de escarcha superficial
    let frost_pattern = uniforms.noise.get_noise_3d(
        position.x * 400.0 + time * 0.1,
        position.y * 400.0,
        position.z * 400.0
    ).abs();

    // Color base con capas de hielo
    let ice_layers = ice_base * 0.7 + ice_detail * 0.3;
    let mut final_color = ice_color.lerp(&deep_ice_color, ice_layers);

    // Sistema de grietas mejorado
    let crack_pattern = cracks_primary * 0.6 + cracks_secondary * 0.4;
    if crack_pattern > 0.65 {
        let crack_intensity = (crack_pattern - 0.65) * 2.5;
        final_color = final_color.lerp(&crack_color, crack_intensity);
        
        // Efecto de profundidad en las grietas
        if crack_pattern > 0.85 {
            final_color = final_color.lerp(&deep_blue, (crack_pattern - 0.85) * 3.0);
        }
    }

    // Cristales de hielo con brillos
    let crystal_pattern = crystals_large * 0.6 + crystals_small * 0.4;
    if crystal_pattern > 0.75 {
        let sparkle = (time * 5.0 + position.magnitude() * 10.0).sin() * 0.5 + 0.5;
        final_color = final_color.lerp(&crystal_glow, (crystal_pattern - 0.75) * 3.0 * sparkle);
    }

    // Efecto de aurora en el hielo
    if aurora_effect > 0.7 {
        final_color = final_color.lerp(&aurora_ice, (aurora_effect - 0.7) * 1.5);
    }

    // Patrón de escarcha en la superficie
    if frost_pattern > 0.9 {
        let frost_intensity = (frost_pattern - 0.9) * 10.0;
        final_color = final_color.lerp(&frost_white, frost_intensity);
    }

    // Efecto de profundidad y atmósfera
    let depth = uniforms.noise.get_noise_3d(
        position.x * 2.0,
        position.y * 2.0,
        position.z * 2.0
    ).abs();

    // Variación del crepúsculo en los polos
    let twilight = (position.y * 2.0).abs();
    if twilight > 0.8 {
        final_color = final_color.lerp(&twilight_ice, (twilight - 0.8) * 2.0);
    }

    // Ajuste final de intensidad con variación de profundidad
    let depth_intensity = 1.0 - (depth * 0.3);
    final_color * fragment.intensity * depth_intensity
}
fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let position = fragment.vertex_position;
  let time = uniforms.time as f32 * 0.001;

  let base_color = Color::new(180, 180, 180);  // Gris claro
  let crater_color = Color::new(100, 100, 100); // Gris oscuro
  let dust_color = Color::new(150, 150, 150);   // Gris medio

  // Patrón base de cráteres
  let craters = uniforms.noise.get_noise_3d(
      position.x * 150.0,
      position.y * 150.0,
      position.z * 150.0
  ).abs();

  // Patrón de polvo lunar
  let dust = uniforms.noise.get_noise_3d(
      position.x * 80.0 + time,
      position.y * 80.0,
      position.z * 80.0
  );

  // Detalles de la superficie
  let surface_details = uniforms.noise.get_noise_3d(
      position.x * 200.0,
      position.y * 200.0,
      position.z * 200.0
  ).abs();

  let mut final_color = base_color;

  // Aplicar cráteres
  if craters > 0.7 {
      final_color = final_color.lerp(&crater_color, (craters - 0.7) * 2.0);
  }

  // Aplicar polvo lunar
  final_color = final_color.lerp(&dust_color, dust.abs() * 0.2);

  // Añadir detalles de superficie
  if surface_details > 0.8 {
      final_color = final_color.lerp(&crater_color, (surface_details - 0.8) * 0.5);
  }

  final_color * fragment.intensity
}

// Planeta Oceánico
fn ocean_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.01;

    //capas de color
    let deep_ocean = Color::new(0, 51, 102);     
    let shallow_water = Color::new(0, 153, 204); 
    let coral_reef = Color::new(64, 224, 208);   
    let surface_foam = Color::new(240, 255, 255);

    // Patrones de oleaje
    let waves = uniforms.noise.get_noise_3d(
        position.x * 50.0 + time,
        position.y * 50.0 + time * 0.5,
        position.z * 50.0
    ).abs();

    // Patrón de profundidad
    let depth = uniforms.noise.get_noise_3d(
        position.x * 30.0,
        position.y * 30.0,
        position.z * 30.0
    ).abs();

    // Patrón de corrientes
    let currents = uniforms.noise.get_noise_3d(
        position.x * 20.0 - time * 0.3,
        position.y * 20.0,
        position.z * 20.0
    ).abs();

    let mut final_color = deep_ocean;
    
    if depth < 0.3 {
        final_color = final_color.lerp(&shallow_water, depth + waves * 0.2);
    } else if depth < 0.6 {
        final_color = final_color.lerp(&coral_reef, currents * 0.5);
    }
    
    if waves > 0.7 {
        final_color = final_color.lerp(&surface_foam, (waves - 0.7) * 0.8);
    }

    final_color * fragment.intensity
}
fn nature_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.005;

    let moss_green = Color::new(98, 185, 82);
    let soil_brown = Color::new(121, 85, 61);
    let deep_forest = Color::new(34, 93, 44);
    let misty_fog = Color::new(180, 200, 195);
    let mossy_neon = Color::new(128, 255, 170);
    let rich_bark = Color::new(121, 85, 72);
    let biolum_blue = Color::new(64, 224, 208);
    let golden_pollen = Color::new(255, 223, 128);
    let purple_fungi = Color::new(147, 112, 219);
    let coral_accent = Color::new(255, 127, 80);

    let veg_base = uniforms.noise.get_noise_3d(
        position.x * 3.5 + time * 0.8,
        position.y * 3.5,
        position.z * 3.5
    ).sin() * 0.5 + 0.5;

    let veg_detail = uniforms.noise.get_noise_3d(
        position.x * 8.0 + time * 0.4,
        position.y * 8.0 + time * 0.3,
        position.z * 8.0
    ).sin() * 0.5 + 0.5;

    let vegetation_pattern = veg_base * 0.7 + veg_detail * 0.3;

    let latitude = position.y.asin();
    let biome_mix = (latitude * 3.0).cos() * 0.5 + 0.5;

    let terrain_spiral = ((position.x * 7.0 + time * 1.2).sin() * 
                         (position.y * 7.0 + time).cos() * 
                         (position.z * 7.0 + time * 0.8).sin()).abs();

    let line_pattern1 = (position.x * 10.0 + position.z * 5.0 + time * 1.5).sin() * 0.5 + 0.5;
    let line_pattern2 = (position.y * 15.0 + position.x * 7.0 + time * 1.2).cos() * 0.5 + 0.5;

    let river_pattern = uniforms.noise.get_noise_3d(
        position.x * 5.0 + time * 0.2,
        position.y * 5.0,
        position.z * 5.0
    ).abs();

    let mut final_color = moss_green.lerp(&deep_forest, vegetation_pattern);
    final_color = final_color.lerp(&soil_brown, biome_mix * 0.4);

    if terrain_spiral > 0.4 {
        final_color = final_color.lerp(&rich_bark, (terrain_spiral - 0.4) * 0.8);
    }

    if line_pattern1 > 0.7 {
        final_color = final_color.lerp(&purple_fungi, (line_pattern1 - 0.7) * 1.3);
    }
    if line_pattern2 > 0.6 {
        final_color = final_color.lerp(&coral_accent, (line_pattern2 - 0.6) * 0.8);
    }

    let biolum_pattern = (position.magnitude() * 8.0 + time).sin().abs();
    if biolum_pattern > 0.8 {
        final_color = final_color.lerp(&biolum_blue, (biolum_pattern - 0.8) * 2.0);
    }

    let pollen = uniforms.noise.get_noise_3d(
        position.x * 20.0 + time * 2.0,
        position.y * 20.0 + time * 1.5,
        position.z * 20.0
    ).abs();
    if pollen > 0.93 {
        final_color = final_color.lerp(&golden_pollen, (pollen - 0.93) * 15.0);
    }

    if river_pattern < 0.1 {
        let water_depth = (river_pattern * 10.0).max(0.0);
        final_color = final_color.lerp(&biolum_blue, 1.0 - water_depth);
    }

    let depth_effect = uniforms.noise.get_noise_3d(
        position.x * 1.8 + time * 0.1,
        position.y * 1.8,
        position.z * 1.8
    ).abs();
    
    let fog_intensity = (time * 0.5).sin() * 0.1 + 0.3;
    final_color = final_color.lerp(&misty_fog, depth_effect * fog_intensity);

    let height_intensity = (position.y * 2.0).sin() * 0.1 + 1.0;
    final_color * fragment.intensity * height_intensity
}

fn aurora_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.01;

    let pink_base = Color::new(255, 84, 180);
    let purple_flow = Color::new(144, 97, 255);
    let lavender_mist = Color::new(210, 158, 255);
    let cyan_glow = Color::new(99, 231, 255);
    let deep_blue = Color::new(2, 119, 188);
    let neon_pink = Color::new(255, 20, 147);
    let electric_blue = Color::new(45, 226, 230);
    let golden_glow = Color::new(255, 215, 0);

    let aurora_base = uniforms.noise.get_noise_3d(
        position.x * 3.5 + time * 0.6,
        position.y * 3.5 + time * 0.4,
        position.z * 3.5
    ).sin() * 0.5 + 0.5;

    let aurora_detail = uniforms.noise.get_noise_3d(
        position.x * 8.0 + time * 0.3,
        position.y * 8.0 + time * 0.2,
        position.z * 8.0
    ).sin() * 0.5 + 0.5;

    let aurora_pattern = aurora_base * 0.7 + aurora_detail * 0.3;

    let wave_primary = (position.x * 15.0 + position.y * 15.0 + time * 4.0).cos() * 0.5 + 0.5;
    let wave_secondary = (position.x * 25.0 - position.y * 25.0 + time * 3.0).sin() * 0.5 + 0.5;
    let wave_lines = wave_primary * 0.6 + wave_secondary * 0.4;

    let mut final_color = pink_base.lerp(&purple_flow, aurora_pattern);
    
    if wave_lines > 0.6 {
        final_color = final_color.lerp(&cyan_glow, (wave_lines - 0.6) * 1.8);
    }
    
    let spiral = ((position.x.atan2(position.y) * 5.0 + time * 2.0).cos() * 0.5 + 0.5) * 
                 (position.magnitude() * 4.0).sin().abs();
    if spiral > 0.7 {
        final_color = final_color.lerp(&electric_blue, (spiral - 0.7) * 1.5);
    }

    let sparkle = uniforms.noise.get_noise_3d(
        position.x * 30.0 + time * 2.0,
        position.y * 30.0 + time * 2.0,
        position.z * 30.0
    ).abs();
    if sparkle > 0.95 {
        final_color = final_color.lerp(&golden_glow, (sparkle - 0.95) * 20.0);
    }

    let circle_pattern = (position.magnitude() * 8.0 + time * 1.5).sin().abs();
    if circle_pattern > 0.5 {
        final_color = final_color.lerp(&lavender_mist, (circle_pattern - 0.5) * 1.5);
    }

    let neon_curve = ((position.x * 12.0 + time).sin() * 
                      (position.y * 12.0 + time).cos() * 
                      (position.z * 12.0 + time * 0.5).sin()).abs();
    if neon_curve > 0.7 {
        final_color = final_color.lerp(&neon_pink, (neon_curve - 0.7) * 1.8);
    }

    let depth = uniforms.noise.get_noise_3d(
        position.x * 2.0 + time * 0.1,
        position.y * 2.0 + time * 0.1,
        position.z * 2.0
    ).abs();
    
    final_color = final_color.lerp(&deep_blue, depth * 0.5);

    final_color * fragment.intensity * 1.2
}

fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let time = uniforms.time as f32 * 0.005;

 
    let band1_color = Color::new(255, 225, 190); 
    let band2_color = Color::new(210, 160, 110); 
    let band3_color = Color::new(180, 130, 90); 

    let storm_core_color = Color::new(255, 100, 80); 
    let storm_edge_color = Color::new(255, 140, 100); 


    let bands = uniforms.noise.get_noise_3d(
        position.x * 50.0 + time,
        position.y * 15.0 + time * 0.2,
        position.z * 50.0,
    );

    let secondary_bands = uniforms.noise.get_noise_3d(
        position.x * 25.0 + time * 0.5,
        position.y * 10.0 + time * 0.1,
        position.z * 25.0,
    );


    let storm = uniforms.noise.get_noise_3d(
        (position.x + 0.5) * 150.0,
        (position.y + 0.5) * 150.0,
        time,
    ).abs();

    let turbulence = uniforms.noise.get_noise_3d(
        position.x * 100.0 + time * 2.0,
        position.y * 100.0,
        position.z * 100.0,
    ).abs();


    let base_band_color = if bands > 0.2 {
        band1_color
    } else if secondary_bands > 0.0 {
        band2_color
    } else {
        band3_color
    };

    let mut final_color = if storm > 0.5 && position.x > 0.0 && position.y > 0.0 {
        storm_core_color.lerp(&storm_edge_color, (storm - 0.5) * 2.0)
    } else {
        base_band_color
    };

    final_color = final_color.lerp(&band3_color, turbulence * 0.3);

    final_color * fragment.intensity
}
