use num_traits::{ One, Float };

pub struct Loader< 'a, T > {
    pub index: usize  ,
    pub data : &'a [T],
}

impl< 'a, T > Loader< 'a, T > {
    pub fn new(data: &'a [T]) -> Self {
        Self {
            index: 0,
            data,
        }
    }

    pub fn load< L: Loadable< T > > (&mut self, buffer: &mut L) {
        buffer.load(self);
    }
}

pub trait Loadable< T > {
    fn load(&mut self, loader: &mut Loader< T >);
}

impl< T: Copy, const N: usize > Loadable< T > for [T; N] {
    fn load(&mut self, loader: &mut Loader< T >) {
        self.copy_from_slice(&loader.data[ loader.index..loader.index + N ]);
        loader.index += N;
    }
}

impl< T: Copy, const N: usize, const M: usize > Loadable< T > for [[T; N]; M] {
    fn load(&mut self, loader: &mut Loader< T >) {
        self.iter_mut().for_each(|row| row.load(loader));
    }
}
