use std::cmp::max;

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use ndarray::Array2;
use rand::Rng;

type Coords = (usize, usize);
type Color = Rgba<u8>;
type Cluster = Vec<Coords>;

// Distance between two pixels
fn dist(p1: Color, p2: Color) -> f32 {
    let tmp1 = (0..3).map(|i| (p1[i] as f32 - p2[i] as f32).abs()).sum::<f32>();
    let tmp2 = 255f32 * 3f32;
    tmp1 / tmp2
}

fn dfs(img: &DynamicImage, visited: &mut Array2<bool>, start: Coords, maxdiff: f32) -> Cluster {
    let mut stack = vec![start];
    let color = img.get_pixel(start.0 as u32, start.1 as u32);
    let mut cluster = Vec::new();
    while let Some(coords) = stack.pop() {
        match visited.get_mut(coords) {
            None | Some(true) => (),
            Some(x) => {
                let color2 = img.get_pixel(coords.0 as u32, coords.1 as u32);
                if dist(color, color2) < maxdiff {
                    *x = true;
                    cluster.push(coords);
                    let (i, j) = coords;
                    stack.push((max(i, 1) - 1, j));
                    stack.push((i + 1, j));
                    stack.push((i, max(j, 1) - 1));
                    stack.push((i, j + 1));
                }
            }
        }
    }

    cluster
}

fn get_clusters(img: &DynamicImage, maxdiff: f32) -> Vec<Cluster> {
    let mut visited = Array2::<bool>::default((img.width() as usize, img.height() as usize));
    let mut clusters = Vec::new();

    (0..img.width() as usize)
        .flat_map(move |i| (0..img.height() as usize).map(move |j| (i, j)))
        .for_each(|coords| {
            if !visited.get(coords.clone()).unwrap() {
                clusters.push(dfs(img, &mut visited, coords, maxdiff));
            }
        });
    clusters
}

fn randcolor() -> Color {
    let mut rng = rand::thread_rng();

    #[allow(deprecated)]  // It suggests Rgba::new as an alternative but that doesn't exist yet
    Color::from_channels(rng.gen(), rng.gen(), rng.gen(), 255)
}

fn apply_color(img: &mut DynamicImage, color: Color, coords: &[Coords]) {
    coords.into_iter().for_each(|(i, j)| {
        img.put_pixel(*i as u32, *j as u32, color);
    });
}

pub fn segment(img: &DynamicImage, maxdiff: f32) -> DynamicImage {
    let clusters = get_clusters(img, maxdiff);
    let mut result = img.clone();
    clusters.into_iter().for_each(|cluster| {
        apply_color(&mut result, randcolor(), &cluster);
    });
    result
}
