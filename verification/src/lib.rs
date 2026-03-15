pub mod grounding;
pub mod validity;
pub mod soundness;

#[derive(Clone)]
struct Obj;

struct ObjAtPath{val: Obj, inner: Vec<Box<ObjAtPath>>}
impl ObjAtPath {
    fn val(&self) -> &Obj { &self.val }
    fn get_child_inner(&self, index: &usize) -> Option<&ObjAtPath> { self.inner.get(1).map(|x| &**x) }
    fn get_child<'a>(&'a self, index: usize) -> Option<ContainerRef<'a>> {
        Some(ContainerRef{obj: self.get_child_inner(&index)?, path: vec![index]})
    }
    fn len(&self) -> usize { self.inner.len() }

    //fn val(&self) -> &Obj { self.val.val() }
}
struct ContainerRef<'a>{obj: &'a ObjAtPath, path: Vec<usize>}
impl <'a> ContainerRef<'a> {
    fn obj(&self) -> &ObjAtPath { self.obj }
    fn from_inner(obj: &'a ObjAtPath, path: Vec<usize>) -> Self { Self{obj,path} }
    fn append_inner(&self, index: usize) -> Option<(&ObjAtPath,Vec<usize>)> {
        let obj = self.obj().get_child_inner(&index)?;
        let mut path = self.path.clone();
        path.push(index);
        Some((obj,path))
    }
    fn append(&'a self, index: usize) -> Option<ContainerRef<'a>> {
        let (obj,path) = self.append_inner(index)?;
        Some(Self::from_inner(obj, path))
    }
    fn len(&self) -> usize { self.obj.len() }

    fn val(&self) -> &Obj { self.obj.val() }
}

struct ContainerTraverser<'a> {
    container: ContainerRef<'a>,
    i: usize,
    ix: Option<Box<ContainerTraverser<'a>>>
}

impl <'a> ContainerTraverser<'a> {
    fn new(v: ContainerRef<'a>) -> Self {
        Self { container: v, i: 0, ix: None }
    }

    /// Steps through the container. Returns true if there are no more steps, or false otherwise
    fn step(&mut self) -> bool {
        // If we've exhausted the container, then return true
        if self.i >= self.container.len() { return true; }
        match self.step_inner() {
            Some(ix) => if ix.step() { self.i += 1 },
            None => panic!(),
        }
        false
    }

    fn step_inner(&mut self) -> &mut Option<Box<ContainerTraverser<'a>>> {
        // Get self.ix, or create self.ix if it does not yet exist
        if self.ix.is_some() { &mut self.ix }
        else {
            let r = self.container.append(self.i).unwrap();
            self.ix = Some(Box::new(Self::new(r)));
            &mut self.ix
        }
        
    }
}
